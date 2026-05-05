import type { ReactNode } from "react";

interface BtnProps {
  variant?: "default" | "primary" | "ghost";
  danger?: boolean;
  children: ReactNode;
}

export function Btn({ children, variant = "default", danger }: BtnProps) {
  const base =
    "rounded-md px-3 py-[7px] text-[13px] font-medium transition-colors cursor-pointer";

  if (danger) {
    return (
      <button
        className={`${base} border border-danger/40 bg-transparent text-danger hover:bg-danger/10`}
      >
        {children}
      </button>
    );
  }

  const styles = {
    default:
      "border border-border-strong bg-surface-2 text-text hover:bg-surface",
    primary: "border border-accent bg-accent text-accent-fg hover:opacity-90",
    ghost:
      "border border-transparent bg-transparent text-text-muted hover:bg-surface-2 hover:text-text",
  };

  return <button className={`${base} ${styles[variant]}`}>{children}</button>;
}
