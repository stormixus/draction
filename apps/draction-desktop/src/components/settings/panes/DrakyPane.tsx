import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { Btn } from "../Btn";
import { Chip } from "../Chip";
import { DrakySprite } from "../DrakySprite";
import { useSettingsStore } from "../../../stores/settingsStore";
import { trOptions, useI18n } from "../../../lib/i18n";

const DRAKY_PERSONALITY_OPTIONS = [
  { value: "friendly", label: "Friendly (default)" },
  { value: "professional", label: "Professional" },
  { value: "silent", label: "Silent" },
];

function sizeToWidth(size: string): string {
  switch (size) {
    case "small": return "33%";
    case "large": return "100%";
    default: return "66%";
  }
}

function widthToSize(pct: string): string {
  const n = parseInt(pct);
  if (n <= 40) return "small";
  if (n >= 90) return "large";
  return "medium";
}

export function DrakyPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);
  const updateSettings = useSettingsStore((s) => s.updateSettings);
  const [previewState, setPreviewState] = useState<"idle" | "wave" | "burp">("idle");

  if (!settings) return null;

  const sizeWidth = sizeToWidth(settings.draky_size);

  return (
    <>
      <PaneHeader
        title="Draky"
        sub={t("Your desktop fairy. Tune how she looks, where she lives and how chatty she is.")}
      />

      <div
        className="mb-6 flex gap-[18px] rounded-xl border border-border p-[18px]"
        style={{
          background:
            "radial-gradient(120% 100% at 30% 0%, rgba(45,212,191,.15), transparent 60%), var(--color-surface)",
        }}
      >
        <div className="flex h-[168px] w-[168px] shrink-0 items-center justify-center rounded-[14px]"
          style={{
            background:
              "radial-gradient(circle at 50% 60%, rgba(45,212,191,.25), transparent 60%)",
          }}
        >
          <DrakySprite
            state={previewState}
            size={150}
            paused={!settings.draky_idle_behaviors || settings.reduce_motion}
          />
        </div>
        <div className="flex flex-1 flex-col justify-center gap-2.5">
          <div className="flex items-center gap-2">
            <h3 className="m-0 text-lg font-semibold">Draky</h3>
            <Chip tone="draky">{t("Idle · awake")}</Chip>
          </div>
          <p className="m-0 text-[13px] leading-relaxed text-text-muted">
            {t("A tiny teal dragon that lives at the corner of your screen. Drop files onto her — she eats them, sorts them, and burps a little log.")}
          </p>
          <div className="mt-1 flex gap-2">
            <Btn
              onClick={() => {
                setPreviewState("wave");
                window.setTimeout(() => setPreviewState("idle"), 900);
              }}
            >
              {t("Wave hello")}
            </Btn>
            <Btn
              variant="ghost"
              onClick={() =>
                updateSettings({
                  draky_size: "medium",
                  draky_overlay_visible: true,
                  draky_snap_to_corner: true,
                })
              }
            >
              {t("Reset position")}
            </Btn>
          </div>
        </div>
      </div>

      <Section title={t("Presence")}>
        <Row label={t("Show overlay")} hint={t("Hide Draky entirely; ingests still work via menu bar.")}>
          <Switch
            aria-label={t("Show overlay")}
            checked={settings.draky_overlay_visible}
            onCheckedChange={(v) => {
              updateSetting("draky_overlay_visible", v);
              invoke("set_overlay_visible", { visible: v }).catch(() => {});
            }}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label={t("Always on top")}>
          <Switch
            aria-label={t("Always on top")}
            checked={settings.draky_always_on_top}
            onCheckedChange={(v) => updateSetting("draky_always_on_top", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label={t("Snap to corner")} hint={t("Lock Draky to whichever corner you drag her to.")}>
          <Switch
            aria-label={t("Snap to corner")}
            checked={settings.draky_snap_to_corner}
            onCheckedChange={(v) => updateSetting("draky_snap_to_corner", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label={t("Size")} hint={t("Affects the overlay window only — not the dashboard preview.")} last>
          <div className="flex items-center gap-2.5">
            <span
              className="cursor-pointer text-[11px] text-text-subtle hover:text-text"
              onClick={() => updateSetting("draky_size", "small")}
            >
              S
            </span>
            <div
              className="relative h-1 w-[120px] cursor-pointer rounded-full bg-surface-2"
              onClick={(e) => {
                const rect = e.currentTarget.getBoundingClientRect();
                const pct = ((e.clientX - rect.left) / rect.width) * 100;
                const snapped = Math.round(pct / 33) * 33;
                updateSetting("draky_size", widthToSize(String(Math.max(33, snapped))));
              }}
            >
              <div
                className="absolute left-0 h-full rounded-full bg-draky"
                style={{ width: sizeWidth }}
              />
              <div
                className="absolute top-[-4px] h-3 w-3 rounded-full bg-white shadow"
                style={{ left: sizeWidth, marginLeft: -6 }}
              />
            </div>
            <span
              className="cursor-pointer text-[11px] text-text-subtle hover:text-text"
              onClick={() => updateSetting("draky_size", "large")}
            >
              L
            </span>
          </div>
        </Row>
      </Section>

      <Section title={t("Personality")}>
        <Row label={t("Voice")} hint={t("How chatty Draky's toasts and tooltips are.")}>
          <Select
            value={settings.draky_personality}
            options={trOptions(DRAKY_PERSONALITY_OPTIONS, t)}
            onChange={(v) => updateSetting("draky_personality", v)}
          />
        </Row>
        <Row label={t("Burp on success")} hint={t("The cute exhale animation after eating files.")}>
          <Switch
            aria-label={t("Burp on success")}
            checked={settings.draky_burp_on_success}
            onCheckedChange={(v) => updateSetting("draky_burp_on_success", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label={t("Idle behaviors")} hint={t("Yawn, stretch, look at the cursor when bored.")} last>
          <Switch
            aria-label={t("Idle behaviors")}
            checked={settings.draky_idle_behaviors}
            onCheckedChange={(v) => updateSetting("draky_idle_behaviors", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
      </Section>

      <Section title={t("File reactions")}>
        <Row label={t("Show file-type munch")} hint={t("Different chew animation per common type.")} last>
          <Switch
            aria-label={t("Show file-type munch")}
            checked={settings.draky_file_type_munch}
            onCheckedChange={(v) => updateSetting("draky_file_type_munch", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
      </Section>
    </>
  );
}
