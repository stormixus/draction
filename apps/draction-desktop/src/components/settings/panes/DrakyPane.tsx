import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { Btn } from "../Btn";
import { Chip } from "../Chip";
import { useSettingsStore } from "../../../stores/settingsStore";

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
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  const sizeWidth = sizeToWidth(settings.draky_size);

  return (
    <>
      <PaneHeader
        title="Draky"
        sub="Your desktop fairy. Tune how she looks, where she lives and how chatty she is."
      />

      {/* Hero: placeholder preview */}
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
          {/* Placeholder for Draky sprite */}
          <div
            className="rounded-full"
            style={{
              width: 150,
              height: 150,
              background: "radial-gradient(circle at 40% 40%, #2dd4bf, #0f766e)",
              boxShadow: "0 0 40px rgba(45,212,191,.3)",
            }}
          />
        </div>
        <div className="flex flex-1 flex-col justify-center gap-2.5">
          <div className="flex items-center gap-2">
            <h3 className="m-0 text-lg font-semibold">Draky</h3>
            <Chip tone="draky">Idle · awake</Chip>
          </div>
          <p className="m-0 text-[13px] leading-relaxed text-text-muted">
            A tiny teal dragon that lives at the corner of your screen. Drop files
            onto her — she eats them, sorts them, and burps a little log.
          </p>
          <div className="mt-1 flex gap-2">
            <Btn>Wave hello</Btn>
            <Btn variant="ghost">Reset position</Btn>
          </div>
        </div>
      </div>

      <Section title="Presence">
        <Row label="Show overlay" hint="Hide Draky entirely; ingests still work via menu bar.">
          <Switch
            aria-label="Show overlay"
            defaultChecked
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label="Always on top">
          <Switch
            aria-label="Always on top"
            checked={settings.draky_always_on_top}
            onCheckedChange={(v) => updateSetting("draky_always_on_top", v)}
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label="Snap to corner" hint="Lock Draky to whichever corner you drag her to.">
          <Switch
            aria-label="Snap to corner"
            defaultChecked
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label="Size" hint="Affects the overlay window only — not the dashboard preview." last>
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

      <Section title="Personality">
        <Row label="Voice" hint="How chatty Draky's toasts and tooltips are.">
          <Select
            value={settings.draky_personality}
            options={DRAKY_PERSONALITY_OPTIONS}
            onChange={(v) => updateSetting("draky_personality", v)}
          />
        </Row>
        <Row label="Burp on success" hint="The cute exhale animation after eating files.">
          <Switch
            aria-label="Burp on success"
            defaultChecked
            className="data-[state=checked]:bg-draky"
          />
        </Row>
        <Row label="Idle behaviors" hint="Yawn, stretch, look at the cursor when bored." last>
          <Switch
            aria-label="Idle behaviors"
            defaultChecked
            className="data-[state=checked]:bg-draky"
          />
        </Row>
      </Section>

      <Section title="File reactions">
        <Row label="Show file-type munch" hint="Different chew animation per common type." last>
          <Switch
            aria-label="Show file-type munch"
            defaultChecked
            className="data-[state=checked]:bg-draky"
          />
        </Row>
      </Section>
    </>
  );
}
