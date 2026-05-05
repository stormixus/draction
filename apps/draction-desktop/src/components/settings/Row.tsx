import type { ReactNode } from "react";

interface RowProps {
  label: ReactNode;
  hint?: ReactNode;
  children: ReactNode;
  last?: boolean;
}

export function Row({ label, hint, children, last }: RowProps) {
  return (
    <div
      className={`flex items-center gap-4 px-4 py-3 ${
        last ? "" : "border-b border-border"
      }`}
    >
      <div className="min-w-0 flex-1">
        <div className="text-[13px] text-text">{label}</div>
        {hint && (
          <div className="mt-[3px] text-[11.5px] leading-snug text-text-subtle">
            {hint}
          </div>
        )}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}
