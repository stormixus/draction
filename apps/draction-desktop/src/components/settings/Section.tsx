import type { ReactNode } from "react";

interface SectionProps {
  title: string;
  desc?: string;
  children: ReactNode;
}

export function Section({ title, desc, children }: SectionProps) {
  return (
    <section className="mb-7">
      <div className="mb-3">
        <h3 className="text-[13px] font-semibold tracking-wide text-text">{title}</h3>
        {desc && (
          <p className="mt-[3px] text-xs leading-relaxed text-text-subtle">{desc}</p>
        )}
      </div>
      <div className="overflow-hidden rounded-[10px] border border-border bg-surface">
        {children}
      </div>
    </section>
  );
}
