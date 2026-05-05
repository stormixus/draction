import type { ReactNode } from "react";

interface InputProps {
  value: ReactNode;
  mono?: boolean;
  width?: number;
  editable?: boolean;
  onChange?: (value: string) => void;
}

export function Input({ value, mono, width = 220, editable = false, onChange }: InputProps) {
  if (editable && onChange) {
    return (
      <input
        type="text"
        value={String(value)}
        onChange={(e) => onChange(e.target.value)}
        className={`rounded-md border border-border-strong bg-bg px-2.5 py-[7px] text-[13px] text-text outline-none
          focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus
          ${mono ? "font-mono" : ""}`}
        style={{ width }}
      />
    );
  }

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
