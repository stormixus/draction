import type { ReactNode } from "react";

interface InputProps {
  value: ReactNode;
  mono?: boolean;
  width?: number;
}

export function Input({ value, mono, width = 220 }: InputProps) {
  return (
    <div
      className={`rounded-md border border-border-strong bg-bg px-2.5 py-[7px] text-[13px] text-text ${
        mono ? "font-mono" : ""
      }`}
      style={{ width }}
    >
      {value}
    </div>
  );
}
