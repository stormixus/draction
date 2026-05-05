import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Input } from "../Input";
import { Select } from "../Select";
import { Btn } from "../Btn";
import { useSettingsStore } from "../../../stores/settingsStore";

const LOG_LEVEL_OPTIONS = [
  { value: "trace", label: "trace" },
  { value: "debug", label: "debug" },
  { value: "info", label: "info" },
  { value: "warn", label: "warn" },
  { value: "error", label: "error" },
];

export function AdvancedPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title="Advanced" sub="For power users — most folks should leave these alone." />

      <Section title="Storage">
        <Row label="Database" hint="Events, runs and audit trail.">
          <div className="flex items-center gap-2">
            <Input value={settings.db_path} mono width={240} />
            <Btn>Reveal</Btn>
          </div>
        </Row>
        <Row label="Rules file">
          <Input value="~/Draction/rules.json" mono width={240} />
        </Row>
        <Row label="Workflows file" last>
          <Input value="~/Draction/workflows.json" mono width={240} />
        </Row>
      </Section>

      <Section title="Logs">
        <Row label="Log level">
          <Select
            value={settings.log_level}
            options={LOG_LEVEL_OPTIONS}
            onChange={(v) => updateSetting("log_level", v)}
          />
        </Row>
        <Row label="Persist logs">
          <Select
            value="14_days"
            options={[{ value: "14_days", label: "14 days" }]}
          />
        </Row>
        <Row label="Open log folder" last>
          <Btn>Open</Btn>
        </Row>
      </Section>

      <Section title="Reset">
        <Row label="Reset to defaults" hint="Restores General, Inbox and Draky panes only.">
          <Btn>Reset</Btn>
        </Row>
        <Row label="Clear database" hint="Removes all events, runs and undo history.">
          <Btn danger>Clear…</Btn>
        </Row>
        <Row label="Erase Draction" hint="Deletes ~/Draction entirely. Cannot be undone." last>
          <Btn danger>Erase…</Btn>
        </Row>
      </Section>
    </>
  );
}
