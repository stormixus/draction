import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { useSettingsStore } from "../../../stores/settingsStore";
import { SUPPORTED_LANGUAGES, trOptions, useI18n } from "../../../lib/i18n";

const THEME_OPTIONS = [
  { value: "system", label: "System (Dark)" },
  { value: "dark", label: "Dark" },
  { value: "light", label: "Light" },
];

const ACCENT_COLORS = [
  { value: "#34d399", label: "Emerald" },
  { value: "#2dd4bf", label: "Teal" },
  { value: "#a78bfa", label: "Violet" },
  { value: "#f472b6", label: "Pink" },
  { value: "#fbbf24", label: "Amber" },
];

const DATE_FORMAT_OPTIONS = [
  { value: "YYYY-MM-DD", label: "2026-05-04" },
  { value: "MM/DD/YYYY", label: "05/04/2026" },
  { value: "DD/MM/YYYY", label: "04/05/2026" },
];

export function GeneralPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  const accentIndex = ACCENT_COLORS.findIndex((c) => c.value === settings.accent_color);

  return (
    <>
      <PaneHeader title={t("General")} sub={t("App-wide preferences.")} />

      <Section title={t("Startup")}>
        <Row label={t("Launch at login")} hint={t("Draction starts when you sign in to your Mac.")}>
          <Switch
            aria-label={t("Launch at login")}
            checked={settings.launch_at_login}
            onCheckedChange={(v) => updateSetting("launch_at_login", v)}
          />
        </Row>
        <Row label={t("Show Draky on launch")} hint={t("The desktop overlay appears immediately.")}>
          <Switch
            aria-label={t("Show Draky on launch")}
            checked={settings.show_draky_on_launch}
            onCheckedChange={(v) => updateSetting("show_draky_on_launch", v)}
          />
        </Row>
        <Row label={t("Run minimized")} hint={t("Hide the dashboard window; only the overlay stays visible.")} last>
          <Switch
            aria-label={t("Run minimized")}
            checked={settings.run_minimized}
            onCheckedChange={(v) => updateSetting("run_minimized", v)}
          />
        </Row>
      </Section>

      <Section title={t("Appearance")}>
        <Row label={t("Theme")} hint={t("Soft-Pro light is in preview.")}>
          <Select
            value={settings.theme}
            options={trOptions(THEME_OPTIONS, t)}
            onChange={(v) => updateSetting("theme", v)}
          />
        </Row>
        <Row label={t("Accent color")} hint={t("Used by buttons, switches and Draky's glow.")}>
          <div className="flex gap-1.5">
            {ACCENT_COLORS.map((c, i) => (
              <span
                key={c.value}
                className="h-[22px] w-[22px] cursor-pointer rounded-full transition-transform hover:scale-110"
                style={{
                  background: c.value,
                  border:
                    i === accentIndex
                      ? "2px solid #fff"
                      : "1px solid var(--color-border-strong)",
                }}
                onClick={() => updateSetting("accent_color", c.value)}
              />
            ))}
          </div>
        </Row>
        <Row label={t("Reduce motion")} hint={t("Drops Draky's idle bobbing and most transitions.")} last>
          <Switch
            aria-label={t("Reduce motion")}
            checked={settings.reduce_motion}
            onCheckedChange={(v) => updateSetting("reduce_motion", v)}
          />
        </Row>
      </Section>

      <Section title={t("Language & region")}>
        <Row label={t("Display language")}>
          <Select
            value={settings.language}
            options={SUPPORTED_LANGUAGES.map((language) => ({
              value: language.value,
              label: language.nativeLabel,
            }))}
            onChange={(v) => updateSetting("language", v)}
          />
        </Row>
        <Row label={t("Date format")} last>
          <Select
            value={settings.date_format}
            options={DATE_FORMAT_OPTIONS}
            onChange={(v) => updateSetting("date_format", v)}
          />
        </Row>
      </Section>
    </>
  );
}
