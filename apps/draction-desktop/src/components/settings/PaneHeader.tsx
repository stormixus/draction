interface PaneHeaderProps {
  title: string;
  sub?: string;
}

export function PaneHeader({ title, sub }: PaneHeaderProps) {
  return (
    <header className="mb-5">
      <h2 className="text-xl font-semibold tracking-tight">{title}</h2>
      {sub && <p className="mt-1 text-[13px] text-text-muted">{sub}</p>}
    </header>
  );
}
