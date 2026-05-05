import type { ReactNode } from "react";

interface SelectOption {
  value: string;
  label: ReactNode;
}

interface SelectProps {
  value: string;
  options: SelectOption[];
  onChange?: (value: string) => void;
}

export function Select({ value, options, onChange }: SelectProps) {
  return (
    <div className="relative inline-flex min-w-[140px]">
      <select
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        className="w-full appearance-none rounded-md border border-border-strong bg-surface-2 px-2.5 py-1.5 pr-7 text-[13px] text-text outline-none cursor-pointer
          focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus"
      >
        {options.map((opt) => (
          <option key={opt.value} value={opt.value}>
            {typeof opt.label === "string" ? opt.label : opt.value}
          </option>
        ))}
      </select>
      <span className="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 text-[10px] text-text-subtle">
        &#9662;
      </span>
    </div>
  );
}
