import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";
import { getCurrentWindow, getAllWindows } from "@tauri-apps/api/window";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type DrakyState = "idle" | "hover" | "processing" | "success" | "error";

interface IngestResult {
  original: string;
  inbox_path: string;
  size_bytes: number;
  sha256: string;
  action: string | null;
}

// Sprite sheet: 8x8 grid, 4096x4096, each cell 512x512
const COLS = 8;
const ROWS = 8;

// Ping-pong: [1,2,3,4] → [1,2,3,4,3,2]
function pingPong(frames: [number, number][]): [number, number][] {
  if (frames.length <= 2) return frames;
  return [...frames, ...frames.slice(1, -1).reverse()];
}

// Raw frames per state
const RAW_FRAMES: Record<DrakyState, [number, number][]> = {
  idle: [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
  hover: [[1, 4], [1, 5], [1, 6], [1, 7]],
  processing: [[2, 4], [2, 5], [3, 0], [3, 1], [3, 2], [1, 4], [1, 5]],
  success: [[3, 5], [3, 6], [7, 3], [7, 4]],
  error: [[1, 6], [1, 7], [7, 0]],
};

// Ping-pong sequences
const SPRITE_MAP: Record<DrakyState, [number, number][]> = {
  idle: pingPong(RAW_FRAMES.idle),           // 10 frames
  hover: pingPong(RAW_FRAMES.hover),         // 6 frames
  processing: pingPong(RAW_FRAMES.processing), // 12 frames
  success: pingPong(RAW_FRAMES.success),     // 6 frames
  error: pingPong(RAW_FRAMES.error),         // 4 frames
};

// Per-frame timing in ms (last frame gets a hold/pose pause)
function getFrameDelay(state: DrakyState, index: number, total: number): number {
  const isLast = index === total - 1;
  switch (state) {
    case "idle":       return 120;
    case "hover":      return 90;
    case "processing": return 80;
    case "success":    return isLast ? 200 : 70;
    case "error":      return isLast ? 200 : 70;
  }
}

async function openMainWindow() {
  const windows = await getAllWindows();
  const main = windows.find((w) => w.label === "main");
  if (main) {
    await main.show();
    await main.setFocus();
  }
}

interface IngestProgress {
  file_name: string;
  bytes_copied: number;
  total_bytes: number;
  percent: number;
}

function OverlayApp() {
  const [state, setState] = useState<DrakyState>("idle");
  const [spriteIndex, setSpriteIndex] = useState(0);
  const [toast, setToast] = useState<string | null>(null);
  const [progress, setProgress] = useState<{ name: string; percent: number } | null>(null);
  const animRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    const unlistenPromise = listen<IngestProgress>("ingest-progress", (event) => {
      setProgress({
        name: event.payload.file_name,
        percent: event.payload.percent,
      });
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    const frames = SPRITE_MAP[state];
    setSpriteIndex(0);

    if (frames.length <= 1) return;

    let cancelled = false;
    let timeoutId: ReturnType<typeof setTimeout>;

    function tick(idx: number) {
      if (cancelled) return;
      const next = (idx + 1) % frames.length;
      const delay = getFrameDelay(state, idx, frames.length);
      timeoutId = setTimeout(() => {
        setSpriteIndex(next);
        tick(next);
      }, delay);
    }
    tick(0);

    return () => {
      cancelled = true;
      clearTimeout(timeoutId);
    };
  }, [state]);

  const [row, col] = SPRITE_MAP[state][spriteIndex] ?? SPRITE_MAP[state][0];

  // Percentage-based sprite position: snaps instantly, no interpolation
  const bgPosX = (col / (COLS - 1)) * 100;
  const bgPosY = (row / (ROWS - 1)) * 100;

  useEffect(() => {
    const unlistenPromise = getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        setState("hover");
      } else if (event.payload.type === "drop") {
        setState("processing");
        const filePaths = event.payload.paths;
        console.log("Dropped files:", filePaths);

        invoke<IngestResult[]>("ingest_files", { paths: filePaths })
          .then((results) => {
            console.log("Ingested:", results);
            setProgress(null);
            setState("success");
            const actions = results.map((r) => `${r.original}${r.action ? ` (${r.action})` : ""}`).join("\n");
            setToast(`${results.length} file(s):\n${actions}`);
            setTimeout(() => { setState("idle"); setToast(null); }, 3000);
          })
          .catch((err) => {
            console.error("Ingest error:", err);
            setProgress(null);
            setState("error");
            setToast(`Error: ${err}`);
            setTimeout(() => { setState("idle"); setToast(null); }, 4000);
          });
      } else {
        setState("idle");
      }
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <div
      className="flex h-screen w-screen items-center justify-center"
      style={{ background: "transparent" }}
      onDragOver={(e) => e.preventDefault()}
      onDrop={(e) => e.preventDefault()}
    >
      <motion.div
        animate={{ scale: state === "hover" ? 1.1 : 1 }}
        transition={{ type: "spring", stiffness: 400, damping: 25 }}
        onPointerDown={(e) => {
          if (e.button === 0) {
            getCurrentWindow().startDragging();
          }
        }}
        style={{ cursor: "grab" }}
        whileTap={{ cursor: "grabbing" }}
      >
        {/* Regular div for sprite — no motion animation on backgroundPosition */}
        <div
          style={{
            width: 220,
            height: 240,
            backgroundImage: "url(/sprites/draky-sheet.png)",
            backgroundSize: `${COLS * 100}% ${ROWS * 100}%`,
            backgroundPosition: `${bgPosX}% ${bgPosY}%`,
            backgroundRepeat: "no-repeat",
          }}
        />
      </motion.div>

      {state === "processing" && (
        <motion.div
          className="pointer-events-none absolute inset-0 flex items-center justify-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 0.6 }}
        >
          <div className="h-60 w-60 animate-spin rounded-full border-4 border-transparent border-t-emerald-400" />
        </motion.div>
      )}

      {state === "processing" && progress && (
        <motion.div
          className="pointer-events-none absolute bottom-16 left-1/2 -translate-x-1/2 text-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
        >
          <div className="text-xs font-medium text-emerald-400">
            {Math.round(progress.percent)}%
          </div>
        </motion.div>
      )}

      {toast && (
        <motion.div
          className="pointer-events-auto absolute bottom-2 left-1/2 max-w-[280px] -translate-x-1/2 rounded-lg px-3 py-2 text-center text-xs font-medium shadow-lg"
          style={{
            background: state === "error" ? "rgba(239,68,68,0.9)" : "rgba(16,185,129,0.9)",
            color: "white",
          }}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0 }}
        >
          <div>{toast}</div>
          <button
            onClick={openMainWindow}
            className="mt-1 underline opacity-80 hover:opacity-100"
            style={{ cursor: "pointer", background: "none", border: "none", color: "inherit", fontSize: "inherit" }}
          >
            자세히
          </button>
        </motion.div>
      )}
    </div>
  );
}

export default OverlayApp;
