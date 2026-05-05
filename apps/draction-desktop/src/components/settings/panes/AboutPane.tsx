import { PaneHeader } from "../PaneHeader";
import { Section } from "../Section";
import { Row } from "../Row";
import { Btn } from "../Btn";

export function AboutPane() {
  return (
    <>
      <PaneHeader title="About" />

      <div className="mb-4 flex items-center gap-[18px] rounded-xl border border-border bg-surface p-5">
        {/* Placeholder Draky */}
        <div
          className="rounded-full"
          style={{
            width: 110,
            height: 110,
            background: "radial-gradient(circle at 40% 40%, #2dd4bf, #0f766e)",
            boxShadow: "0 0 30px rgba(45,212,191,.3)",
          }}
        />
        <div>
          <div className="text-lg font-semibold">
            Draction{" "}
            <span className="text-[13px] font-normal text-text-subtle">v0.1.4 (build 318)</span>
          </div>
          <div className="mt-1 max-w-[420px] text-[13px] leading-relaxed text-text-muted">
            A tiny, magical fairy living on your desktop. She happily munches your
            files and magically organizes them into folders. Made with love and a lot
            of caffeine.
          </div>
          <div className="mt-3 flex gap-2">
            <Btn variant="primary">Check for updates</Btn>
            <Btn>Release notes</Btn>
            <Btn variant="ghost">View on GitHub →</Btn>
          </div>
        </div>
      </div>

      <Section title="System">
        <Row label="Engine">
          <span className="font-mono text-xs text-text-muted">draction-engine 0.1.4</span>
        </Row>
        <Row label="Tauri">
          <span className="font-mono text-xs text-text-muted">2.1.1</span>
        </Row>
        <Row label="Database schema" last>
          <span className="font-mono text-xs text-text-muted">v3 (2026-04-12)</span>
        </Row>
      </Section>
    </>
  );
}
