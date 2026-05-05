import { useState, useEffect, useRef, useCallback } from "react";
import { motion } from "framer-motion";
import { getCurrentWindow, getAllWindows } from "@tauri-apps/api/window";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// ── Draky states (matches draky-bible.jsx) ────────────────────────
// "processing" = Munching internally, "burp" = Success internally
type DrakyState = "idle" | "hover" | "processing" | "burp" | "error" | "too_big" | "sleep" | "wave";

interface IngestResult {
  original: string;
  inbox_path: string;
  size_bytes: number;
  sha256: string;
  action: string | null;
}

interface IngestProgress {
  file_name: string;
  bytes_copied: number;
  total_bytes: number;
  percent: number;
}

// ── Sprite sheet: 8×8 grid, 4096×4096, each cell 512×512 ──────────
const COLS = 8;
const ROWS = 8;

// Ping-pong: [1,2,3,4] → [1,2,3,4,3,2]
function pingPong(frames: [number, number][]): [number, number][] {
  if (frames.length <= 2) return frames;
  return [...frames, ...frames.slice(1, -1).reverse()];
}

// Raw frames per sprite key — expanded from draky-engine.jsx per task spec
const RAW_FRAMES: Record<string, [number, number][]> = {
  idle:       [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
  hover:      [[1, 4], [1, 5], [1, 6], [1, 7]],
  processing: [[2, 4], [2, 5], [3, 0], [3, 1], [3, 2], [1, 4], [1, 5]],
  burp:       [[3, 5], [3, 6], [7, 3], [7, 4]],
  error:      [[1, 6], [1, 7], [7, 0]],
  sleep:      [[7, 0], [7, 1], [7, 2]],
  too_big:    [[6, 0], [6, 1], [6, 2], [6, 3], [6, 4], [6, 5]],
  wave:       [[0, 6], [1, 0], [1, 1], [1, 2], [1, 3]],
};

// Ping-pong sequences (state key maps 1:1 to sprite key)
const SPRITE_MAP: Record<string, [number, number][]> = Object.fromEntries(
  Object.entries(RAW_FRAMES).map(([k, v]) => [k, pingPong(v)]),
);

// Per-frame timing in ms
const FRAME_MS: Record<string, number> = {
  idle: 120,
  hover: 90,
  processing: 80,
  burp: 200,
  error: 110,
  sleep: 700,
  too_big: 140,
  wave: 130,
};

// ── Voice / copy table (from draky-bible.jsx VOICE) ───────────────
function getDrakyPhrase(
  state: DrakyState,
  fileCount: number,
  errorMsg?: string,
): string | null {
  switch (state) {
    case "burp":
      return fileCount === 1
        ? "Yum! Tucked into Inbox."
        : `Yum! ${fileCount} files sorted.`;
    case "error":
      return errorMsg
        ? `Oops — couldn't finish. ${errorMsg}`
        : "Oops — couldn't finish. Open log?";
    case "too_big":
      return "Oof, big one.";
    case "wave":
      return "Hi! I'm Draky. Drop something on me.";
    default:
      return null;
  }
}

// ── File-type reactions ────────────────────────────────────────────
function getFileReaction(path: string): string | null {
  // Detect folder (trailing slash or no extension)
  if (path.endsWith("/") || path.endsWith("\\")) return "A whole bag!";

  const ext = path.split(".").pop()?.toLowerCase();
  if (ext === path) return null; // no extension — unknown

  switch (ext) {
    case "pdf":
      return "Mmm, paperwork.";
    case "docx":
    case "doc":
      return "Crunchy.";
    case "pptx":
    case "ppt":
      return "So many slides...";
    default:
      return "What is this?";
  }
}

// ── Helpers ────────────────────────────────────────────────────────
async function openMainWindow() {
  const windows = await getAllWindows();
  const main = windows.find((w) => w.label === "main");
  if (main) {
    await main.show();
    await main.setFocus();
  }
}

// ── Component ──────────────────────────────────────────────────────
function OverlayApp() {
  const [state, setState] = useState<DrakyState>("idle");
  const [spriteIndex, setSpriteIndex] = useState(0);
  const [toast, setToast] = useState<string | null>(null);
  const [progress, setProgress] = useState<{
    name: string;
    percent: number;
  } | null>(null);
  const [fileReaction, setFileReaction] = useState<string | null>(null);

  // ── Refs for activity tracking & WS lifecycle ──────────────────
  const lastActivityRef = useRef<number>(Date.now());
  const wsRef = useRef<WebSocket | null>(null);
  const wsReconnectRef = useRef<ReturnType<typeof setTimeout> | null>(
    null,
  );

  const markActivity = useCallback(() => {
    lastActivityRef.current = Date.now();
  }, []);

  // ── Wave on mount (first-run greeting) ──────────────────────────
  useEffect(() => {
    setState("wave");
    const t = setTimeout(() => setState("idle"), 1800);
    return () => clearTimeout(t);
  }, []);

  // ── Sleep timer: 5 min idle → sleep ─────────────────────────────
  useEffect(() => {
    const interval = setInterval(() => {
      const idleMs = Date.now() - lastActivityRef.current;
      setState((prev) => {
        if (prev === "idle" && idleMs >= 5 * 60 * 1000) {
          return "sleep";
        }
        if (prev === "sleep" && idleMs < 5 * 60 * 1000) {
          return "idle";
        }
        return prev;
      });
    }, 15_000); // check every 15 s
    return () => clearInterval(interval);
  }, []);

  // ── WebSocket: connect to EventBus ──────────────────────────────
  useEffect(() => {
    let cancelled = false;

    async function connect() {
      try {
        const port: number = await invoke("get_api_port");
        const token: string = await invoke("get_auth_token");

        if (cancelled) return;

        const ws = new WebSocket(
          `ws://localhost:${port}/ws?token=${token}`,
        );
        wsRef.current = ws;

        ws.onopen = () => {
          console.log("[Draky] WebSocket connected");
        };

        ws.onmessage = (event) => {
          markActivity();
          try {
            const data = JSON.parse(event.data) as {
              type: string;
              file_count?: number;
            };

            switch (data.type) {
              case "EVENT_INGESTED":
              case "RUN_STARTED":
                // Stay in processing; let the drag-drop handler own the transition
                break;
              case "RUN_FINISHED":
                setState("burp");
                setTimeout(() => setState((prev) => prev === "burp" ? "idle" : prev), 3000);
                break;
              case "RUN_FAILED":
                setState("error");
                setTimeout(() => setState((prev) => prev === "error" ? "idle" : prev), 4000);
                break;
            }
          } catch {
            // ignore unparseable messages
          }
        };

        ws.onerror = () => {
          // Will trigger onclose; reconnect there
        };

        ws.onclose = () => {
          console.log("[Draky] WebSocket closed, reconnecting in 5s");
          wsRef.current = null;
          if (!cancelled) {
            wsReconnectRef.current = setTimeout(connect, 5000);
          }
        };
      } catch (err) {
        console.error("[Draky] WS connect error:", err);
        if (!cancelled) {
          wsReconnectRef.current = setTimeout(connect, 5000);
        }
      }
    }

    connect();

    return () => {
      cancelled = true;
      if (wsReconnectRef.current) clearTimeout(wsReconnectRef.current);
      wsRef.current?.close();
    };
  }, [markActivity]);

  // ── Ingest progress listener ────────────────────────────────────
  useEffect(() => {
    const unlistenPromise = listen<IngestProgress>(
      "ingest-progress",
      (event) => {
        setProgress({
          name: event.payload.file_name,
          percent: event.payload.percent,
        });
      },
    );
    return () => {
      unlistenPromise.then((u) => u());
    };
  }, []);

  // ── Sprite animation loop ──────────────────────────────────────
  const spriteKey = state; // 1:1 mapping between DrakyState and sprite key
  const frames = SPRITE_MAP[spriteKey] ?? SPRITE_MAP.idle;

  useEffect(() => {
    setSpriteIndex(0);

    if (frames.length <= 1) return;

    let cancelled = false;
    let timeoutId: ReturnType<typeof setTimeout>;

    let idx = 0;
    function tick() {
      if (cancelled) return;
      const delay = FRAME_MS[spriteKey] ?? 120;
      timeoutId = setTimeout(() => {
        idx = (idx + 1) % frames.length;
        setSpriteIndex(idx);
        tick();
      }, delay);
    }
    tick();

    return () => {
      cancelled = true;
      clearTimeout(timeoutId);
    };
  }, [state, spriteKey, frames.length]);

  const [row, col] = frames[spriteIndex] ?? frames[0];

  // Percentage-based sprite position
  const bgPosX = (col / (COLS - 1)) * 100;
  const bgPosY = (row / (ROWS - 1)) * 100;

  // ── Drag-drop handler (Tauri native events) ────────────────────
  useEffect(() => {
    const unlistenPromise = getCurrentWebview().onDragDropEvent(
      (event) => {
        markActivity();

        if (event.payload.type === "over") {
          setState("hover");
        } else if (event.payload.type === "drop") {
          const filePaths = event.payload.paths;
          console.log("[Draky] Dropped files:", filePaths);

          setState("processing");
          setProgress(null);

          // File-type reaction from first dropped file
          if (filePaths.length > 0) {
            setFileReaction(getFileReaction(filePaths[0]));
          }

          invoke<IngestResult[]>("ingest_files", { paths: filePaths })
            .then((results) => {
              console.log("[Draky] Ingested:", results);
              setProgress(null);
              setFileReaction(null);
              setState("burp");

              setToast(getDrakyPhrase("burp", results.length));
              setTimeout(() => {
                setState((prev) => prev === "burp" ? "idle" : prev);
                setToast(null);
              }, 3000);
            })
            .catch((err) => {
              console.error("[Draky] Ingest error:", err);
              setProgress(null);
              setFileReaction(null);

              const errStr = String(err).toLowerCase();
              // Detect "too big" errors
              if (
                errStr.includes("too large") ||
                errStr.includes("size exceeds") ||
                errStr.includes("too_big") ||
                errStr.includes("file size")
              ) {
                setState("too_big");
                setToast(getDrakyPhrase("too_big", 0));
                setTimeout(() => {
                  setState((prev) => prev === "too_big" ? "idle" : prev);
                  setToast(null);
                }, 2000);
              } else {
                setState("error");
                setToast(getDrakyPhrase("error", 0, String(err)));
                setTimeout(() => {
                  setState((prev) => prev === "error" ? "idle" : prev);
                  setToast(null);
                }, 4000);
              }
            });
        } else {
          // "leave" — go idle unless processing/sleeping
          setState((prev) =>
            prev === "hover" || prev === "idle" ? "idle" : prev,
          );
        }
      },
    );

    return () => {
      unlistenPromise.then((u) => u());
    };
  }, [markActivity]);

  // ── Wobble animation variants (subtle state transitions) ───────
  const wobbleVariants = {
    idle: { rotate: 0, scale: 1 },
    hover: { rotate: 0, scale: 1.1 },
    processing: { rotate: [0, -2, 2, -1, 1, 0], scale: 1 },
    burp: { rotate: [0, -3, 3, 0], scale: [1, 1.08, 1.04, 1] },
    error: { rotate: [0, -3, 3, -2, 2, 0], scale: [1, 1.05, 1] },
    too_big: { rotate: [0, -2, 2, 0], scale: [1, 1.06, 1.03, 1] },
    sleep: { rotate: 0, scale: 1 },
    wave: { rotate: [0, -4, 4, -3, 3, 0], scale: [1, 1.08, 1.05, 1] },
  };

  return (
    <div
      className="flex h-screen w-screen items-center justify-center"
      style={{ background: "transparent" }}
      onDragOver={(e) => e.preventDefault()}
      onDrop={(e) => e.preventDefault()}
    >
      <motion.div
        animate={wobbleVariants[state] ?? wobbleVariants.idle}
        transition={{
          type: "spring",
          stiffness: 400,
          damping: 25,
          duration: state === "idle" || state === "sleep" ? 0.3 : 0.5,
        }}
        onPointerDown={(e) => {
          if (e.button === 0) {
            getCurrentWindow().startDragging();
          }
        }}
        style={{ cursor: "grab" }}
        whileTap={{ cursor: "grabbing" }}
      >
        {/* Draky sprite — static div, no motion on backgroundPosition */}
        <div
          style={{
            width: 220,
            height: 240,
            backgroundImage: "url(/sprites/draky-sheet.png)",
            backgroundSize: `${COLS * 100}% ${ROWS * 100}%`,
            backgroundPosition: `${bgPosX}% ${bgPosY}%`,
            backgroundRepeat: "no-repeat",
            // Subtle glow on active states
            filter:
              state === "processing" || state === "burp"
                ? "drop-shadow(0 0 8px rgba(45,212,191,0.4))"
                : state === "error" || state === "too_big"
                  ? "drop-shadow(0 0 8px rgba(248,113,113,0.35))"
                  : state === "wave"
                    ? "drop-shadow(0 0 8px rgba(45,212,191,0.35))"
                    : "none",
            transition: "filter 0.4s ease",
          }}
        />
      </motion.div>

      {/* Processing spinner ring */}
      {state === "processing" && (
        <motion.div
          className="pointer-events-none absolute inset-0 flex items-center justify-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 0.6 }}
        >
          <div className="h-60 w-60 animate-spin rounded-full border-4 border-transparent border-t-accent" />
        </motion.div>
      )}

      {/* File reaction text (shown during processing) */}
      {state === "processing" && fileReaction && (
        <motion.div
          className="pointer-events-none absolute top-8 left-1/2 -translate-x-1/2 text-center"
          initial={{ opacity: 0, y: -4 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <div className="text-xs font-medium italic text-accent/80">
            {fileReaction}
          </div>
        </motion.div>
      )}

      {/* Progress percent */}
      {state === "processing" && progress && (
        <motion.div
          className="pointer-events-none absolute bottom-16 left-1/2 -translate-x-1/2 text-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
        >
          <div className="text-xs font-medium text-accent">
            {Math.round(progress.percent)}%
          </div>
        </motion.div>
      )}

      {/* Toast notification */}
      {toast && (
        <motion.div
          className={
            "pointer-events-auto absolute bottom-2 left-1/2 max-w-[280px] -translate-x-1/2 rounded-lg px-3 py-2 text-center text-xs font-medium text-white shadow-[var(--shadow-float)] " +
            (state === "error" || state === "too_big"
              ? "bg-danger/90"
              : "bg-success/90")
          }
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0 }}
        >
          <div>{toast}</div>
          {(state === "error" || state === "too_big") && (
            <button
              onClick={openMainWindow}
              className="mt-1 cursor-pointer border-none bg-transparent text-inherit underline opacity-80 hover:opacity-100"
            >
              Open Draction
            </button>
          )}
        </motion.div>
      )}

      {/* Sleep zZ indicator */}
      {state === "sleep" && (
        <motion.div
          className="pointer-events-none absolute right-6 top-6 text-xl font-bold text-accent/40"
          initial={{ opacity: 0, y: -4 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
        >
          zZ
        </motion.div>
      )}
    </div>
  );
}

export default OverlayApp;
