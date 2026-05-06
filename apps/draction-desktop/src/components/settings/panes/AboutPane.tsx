import { PaneHeader } from "../PaneHeader";
import { Section } from "../Section";
import { Row } from "../Row";
import { Btn } from "../Btn";
import { DrakySprite } from "../DrakySprite";
import { useToast } from "../../ui/Toast";
import { useI18n } from "../../../lib/i18n";

export function AboutPane() {
  const t = useI18n();
  const toast = useToast();

  return (
    <>
      <PaneHeader title={t("About")} />

      <div className="mb-4 flex items-center gap-[18px] rounded-xl border border-border bg-surface p-5">
        <DrakySprite state="wave" size={110} />
        <div>
          <div className="text-lg font-semibold">
            Draction{" "}
            <span className="text-[13px] font-normal text-text-subtle">v0.1.4 (build 318)</span>
          </div>
          <div className="mt-1 max-w-[420px] text-[13px] leading-relaxed text-text-muted">
            {t("A tiny, magical fairy living on your desktop. She happily munches your files and magically organizes them into folders. Made with love and a lot of caffeine.")}
          </div>
          <div className="mt-3 flex gap-2">
            <Btn
              variant="primary"
              onClick={() => toast.success(t("Draction is up to date"), { description: t("v0.1.4 is the current local build.") })}
            >
              {t("Check for updates")}
            </Btn>
            <Btn onClick={() => toast.show({ title: t("Release notes"), description: t("No bundled release notes yet.") })}>
              {t("Release notes")}
            </Btn>
            <Btn
              variant="ghost"
              onClick={() => window.open("https://github.com", "_blank", "noopener,noreferrer")}
            >
              {t("View on GitHub →")}
            </Btn>
          </div>
        </div>
      </div>

      <Section title={t("System")}>
        <Row label={t("Engine")}>
          <span className="font-mono text-xs text-text-muted">draction-engine 0.1.4</span>
        </Row>
        <Row label="Tauri">
          <span className="font-mono text-xs text-text-muted">2.1.1</span>
        </Row>
        <Row label={t("Database schema")} last>
          <span className="font-mono text-xs text-text-muted">v3 (2026-04-12)</span>
        </Row>
      </Section>
    </>
  );
}
