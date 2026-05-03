import * as RadixToast from "@radix-ui/react-toast";
import { createContext, useCallback, useContext, useMemo, useState } from "react";
import type { ReactNode } from "react";

export type ToastTone = "default" | "success" | "error";

interface ToastEntry {
  id: number;
  title: ReactNode;
  description?: ReactNode;
  tone: ToastTone;
  duration: number;
  action?: { label: string; onClick: () => void };
}

interface ToastApi {
  show: (entry: Omit<ToastEntry, "id" | "tone" | "duration"> & {
    tone?: ToastTone;
    duration?: number;
  }) => void;
  success: (title: ReactNode, opts?: Partial<Omit<ToastEntry, "id" | "tone" | "title">>) => void;
  error: (title: ReactNode, opts?: Partial<Omit<ToastEntry, "id" | "tone" | "title">>) => void;
}

const ToastContext = createContext<ToastApi | null>(null);

export function useToast(): ToastApi {
  const ctx = useContext(ToastContext);
  if (!ctx) throw new Error("useToast must be used within <ToastProvider>");
  return ctx;
}

const TONE_STYLES: Record<ToastTone, string> = {
  default: "border-border-strong bg-surface text-text",
  success: "border-success/40 bg-surface text-text",
  error: "border-danger/50 bg-surface text-text",
};

const TONE_DOT: Record<ToastTone, string> = {
  default: "bg-text-subtle",
  success: "bg-success",
  error: "bg-danger",
};

export function ToastProvider({ children }: { children: ReactNode }) {
  const [entries, setEntries] = useState<ToastEntry[]>([]);

  const remove = useCallback((id: number) => {
    setEntries((current) => current.filter((e) => e.id !== id));
  }, []);

  const api = useMemo<ToastApi>(() => {
    let nextId = 1;
    const baseShow: ToastApi["show"] = ({ tone = "default", duration, ...rest }) => {
      const id = nextId++;
      setEntries((current) => [
        ...current,
        {
          id,
          tone,
          duration: duration ?? (tone === "error" ? 6000 : 3000),
          ...rest,
        },
      ]);
    };
    return {
      show: baseShow,
      success: (title, opts) => baseShow({ title, tone: "success", ...opts }),
      error: (title, opts) => baseShow({ title, tone: "error", ...opts }),
    };
  }, []);

  return (
    <ToastContext.Provider value={api}>
      <RadixToast.Provider swipeDirection="down">
        {children}
        {entries.map((entry) => (
          <RadixToast.Root
            key={entry.id}
            duration={entry.duration}
            onOpenChange={(open) => {
              if (!open) remove(entry.id);
            }}
            className={
              "pointer-events-auto rounded-lg border px-3 py-2 shadow-[var(--shadow-float)] " +
              "data-[state=open]:animate-in data-[state=closed]:animate-out " +
              "data-[state=closed]:fade-out-80 data-[state=open]:slide-in-from-bottom-2 " +
              TONE_STYLES[entry.tone]
            }
          >
            <div className="flex items-start gap-2">
              <span className={"mt-1 h-2 w-2 shrink-0 rounded-full " + TONE_DOT[entry.tone]} />
              <div className="min-w-0 flex-1">
                <RadixToast.Title className="text-xs font-medium">
                  {entry.title}
                </RadixToast.Title>
                {entry.description && (
                  <RadixToast.Description className="mt-0.5 text-xs text-text-muted">
                    {entry.description}
                  </RadixToast.Description>
                )}
                {entry.action && (
                  <RadixToast.Action
                    asChild
                    altText={entry.action.label}
                  >
                    <button
                      onClick={entry.action.onClick}
                      className="mt-1 text-xs font-medium text-accent underline-offset-2 hover:underline"
                    >
                      {entry.action.label}
                    </button>
                  </RadixToast.Action>
                )}
              </div>
              <RadixToast.Close
                aria-label="Dismiss notification"
                className="ml-2 text-text-subtle hover:text-text leading-none"
              >
                ×
              </RadixToast.Close>
            </div>
          </RadixToast.Root>
        ))}
        <RadixToast.Viewport className="fixed bottom-3 left-1/2 z-50 flex max-w-[320px] -translate-x-1/2 flex-col gap-2 outline-none" />
      </RadixToast.Provider>
    </ToastContext.Provider>
  );
}
