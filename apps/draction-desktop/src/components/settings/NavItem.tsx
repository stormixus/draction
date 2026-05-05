import type { ReactNode } from "react";

interface NavItemProps {
  icon: ReactNode;
  label: string;
  badge?: string;
  active?: boolean;
  draky?: boolean;
  onClick?: () => void;
}

export function NavItem({ icon, label, badge, active, draky, onClick }: NavItemProps) {
  return (
    <button
      onClick={onClick}
      className={`flex w-full items-center gap-2.5 rounded-md px-2.5 py-[7px] text-left text-[13px] transition-colors cursor-pointer ${
        active
          ? "bg-surface-2 text-text"
          : "bg-transparent text-text-muted hover:text-text"
      }`}
    >
      <span
        className={`inline-flex h-[18px] w-[18px] items-center justify-center ${
          active
            ? draky
              ? "text-draky"
              : "text-accent"
            : "text-text-subtle"
        }`}
      >
        {icon}
      </span>
      <span className="flex-1">{label}</span>
      {badge && (
        <span className="rounded-full border border-border bg-surface-2 px-[6px] py-[2px] text-[10px] text-text-subtle">
          {badge}
        </span>
      )}
    </button>
  );
}
