import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { useSettingsStore } from "../../../stores/settingsStore";

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

const LANGUAGE_OPTIONS = [
  { value: "ko", label: "한국어" },
  { value: "en", label: "English" },
  { value: "ja", label: "日本語" },
];

const DATE_FORMAT_OPTIONS = [
  { value: "YYYY-MM-DD", label: "2026-05-04" },
  { value: "MM/DD/YYYY", label: "05/04/2026" },
  { value: "DD/MM/YYYY", label: "04/05/2026" },
];

export function GeneralPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  const accentIndex = ACCENT_COLORS.findIndex((c) => c.value === settings.accent_color);

  return (
    <>
      <PaneHeader title="General" sub="App-wide preferences." />

      <Section title="Startup">
        <Row label="Launch at login" hint="Draction starts when you sign in to your Mac.">
          <Switch
            aria-label="Launch at login"
            checked={settings.launch_at_login}
            onCheckedChange={(v) => updateSetting("launch_at_login", v)}
          />
        </Row>
        <Row label="Show Draky on launch" hint="The desktop overlay appears immediately.">
          <Switch
            aria-label="Show Draky on launch"
            checked={settings.show_draky_on_launch}
            onCheckedChange={(v) => updateSetting("show_draky_on_launch", v)}
          />
        </Row>
        <Row label="Run minimized" hint="Hide the dashboard window; only the overlay stays visible." last>
          <Switch
            aria-label="Run minimized"
            checked={settings.run_minimized}
            onCheckedChange={(v) => updateSetting("run_minimized", v)}
          />
        </Row>
      </Section>

      <Section title="Appearance">
        <Row label="Theme" hint="Soft-Pro light is in preview.">
          <Select
            value={settings.theme}
            options={THEME_OPTIONS}
            onChange={(v) => updateSetting("theme", v)}
          />
        </Row>
        <Row label="Accent color" hint="Used by buttons, switches and Draky's glow.">
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
        <Row label="Reduce motion" hint="Drops Draky's idle bobbing and most transitions." last>
          <Switch
            aria-label="Reduce motion"
            checked={settings.reduce_motion}
            onCheckedChange={(v) => updateSetting("reduce_motion", v)}
          />
        </Row>
      </Section>

      <Section title="Language & region">
        <Row label="Display language">
          <Select
            value={settings.language}
            options={LANGUAGE_OPTIONS}
            onChange={(v) => updateSetting("language", v)}
          />
        </Row>
        <Row label="Date format" last>
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
