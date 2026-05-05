import type { ReactNode } from "react";

interface SelectProps {
  value: ReactNode;
}

export function Select({ value }: SelectProps) {
  return (
    <div className="inline-flex min-w-[140px] items-center gap-1.5 rounded-md border border-border-strong bg-surface-2 px-2.5 py-1.5 text-[13px] text-text">
      <span className="flex-1">{value}</span>
      <span className="text-[10px] text-text-subtle">&#9662;</span>
    </div>
  );
}
