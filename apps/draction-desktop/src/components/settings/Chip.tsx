import type { ReactNode } from "react";

interface ChipProps {
  tone?: "neutral" | "accent" | "draky" | "warn";
  children: ReactNode;
}

export function Chip({ children, tone = "neutral" }: ChipProps) {
  const t = {
    neutral: "bg-surface-2 text-text-muted border-border",
    accent: "bg-accent/10 text-accent border-accent/30",
    draky: "bg-draky/10 text-draky border-draky/30",
    warn: "bg-warning/10 text-warning border-warning/30",
  };

  return (
    <span
      className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-[11px] font-medium ${t[tone]}`}
    >
      {children}
    </span>
  );
}
