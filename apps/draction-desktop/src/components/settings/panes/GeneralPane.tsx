import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";

export function GeneralPane() {
  return (
    <>
      <PaneHeader title="General" sub="App-wide preferences." />

      <Section title="Startup">
        <Row label="Launch at login" hint="Draction starts when you sign in to your Mac.">
          <Switch aria-label="Launch at login" defaultChecked />
        </Row>
        <Row label="Show Draky on launch" hint="The desktop overlay appears immediately.">
          <Switch aria-label="Show Draky on launch" defaultChecked />
        </Row>
        <Row label="Run minimized" hint="Hide the dashboard window; only the overlay stays visible." last>
          <Switch aria-label="Run minimized" />
        </Row>
      </Section>

      <Section title="Appearance">
        <Row label="Theme" hint="Soft-Pro light is in preview.">
          <Select value="System (Dark)" />
        </Row>
        <Row label="Accent color" hint="Used by buttons, switches and Draky's glow.">
          <div className="flex gap-1.5">
            {["#34d399", "#2dd4bf", "#a78bfa", "#f472b6", "#fbbf24"].map((c, i) => (
              <span
                key={c}
                className="h-[22px] w-[22px] cursor-pointer rounded-full"
                style={{
                  background: c,
                  border: i === 0 ? "2px solid #fff" : "1px solid var(--color-border-strong)",
                }}
              />
            ))}
          </div>
        </Row>
        <Row label="Reduce motion" hint="Drops Draky's idle bobbing and most transitions." last>
          <Switch aria-label="Reduce motion" />
        </Row>
      </Section>

      <Section title="Language & region">
        <Row label="Display language">
          <Select value="한국어" />
        </Row>
        <Row label="Date format" last>
          <Select value="2026-05-04" />
        </Row>
      </Section>
    </>
  );
}
