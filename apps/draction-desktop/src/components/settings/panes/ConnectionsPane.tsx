import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Input } from "../Input";
import { Btn } from "../Btn";
import { Chip } from "../Chip";
import { useSettingsStore } from "../../../stores/settingsStore";

export function ConnectionsPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title="Connections" sub="Local services that can talk to Draction." />

      <Section title="Local API">
        <Row label="HTTP port">
          <Input value={String(settings.api_port)} mono width={100} />
        </Row>
        <Row label="Bind address" hint="Localhost only — never exposed to the network.">
          <Input value="127.0.0.1" mono width={140} />
        </Row>
        <Row label="Bearer token" hint="Required for OpenClaw and other clients." last>
          <div className="flex items-center gap-2">
            <Input value="dr_3f8a··········9c2" mono width={200} />
            <Btn>Rotate</Btn>
          </div>
        </Row>
      </Section>

      <Section title="OpenClaw bridge" desc="The optional AI peer that suggests rules.">
        <Row label="Status">
          <Chip tone={settings.openclaw_paired ? "accent" : "warn"}>
            {settings.openclaw_paired ? "Connected" : "Not connected"}
          </Chip>
        </Row>
        <Row label="Auto-suggest rules" hint="Draky asks 'always do this?' after each ingest.">
          <Switch aria-label="Auto-suggest rules" />
        </Row>
        <Row label="Send file metadata" hint="Filename, size, mime — never file contents." last>
          <Switch aria-label="Send file metadata" defaultChecked />
        </Row>
      </Section>

      <Section title="Pairing">
        <div className="flex items-center gap-[18px] p-[18px]">
          <div
            className="h-24 w-24 rounded-lg"
            style={{
              backgroundImage: "repeating-conic-gradient(#000 0 25%, #fff 0 50%)",
              backgroundSize: "12px 12px",
            }}
          />
          <div className="flex-1">
            <div className="mb-1 text-[13px] font-semibold">Pair OpenClaw</div>
            <div className="mb-2.5 text-xs leading-relaxed text-text-muted">
              Scan from OpenClaw's "Add bridge" screen, or enter the code manually.
              Pairing approval is required from this device.
            </div>
            <div className="flex gap-2">
              <Input
                value={settings.pairing_code}
                mono
                width={140}
                editable
                onChange={(v) => updateSetting("pairing_code", v)}
              />
              <Btn variant="primary">Approve incoming</Btn>
            </div>
          </div>
        </div>
      </Section>
    </>
  );
}
