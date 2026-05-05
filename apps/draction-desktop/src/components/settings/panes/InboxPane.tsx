import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { Input } from "../Input";
import { useSettingsStore } from "../../../stores/settingsStore";

const CONFLICT_OPTIONS = [
  { value: "append_timestamp", label: "Append timestamp" },
  { value: "overwrite", label: "Overwrite" },
  { value: "skip", label: "Skip" },
];

const UNDO_WINDOW_OPTIONS = [
  { value: "10", label: "10 seconds" },
  { value: "30", label: "30 seconds" },
  { value: "60", label: "60 seconds" },
  { value: "120", label: "2 minutes" },
];

export function InboxPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title="Inbox" sub="Where dropped files land before workflows pick them up." />

      <Section title="Inbox folder">
        <Row label="Location" hint="Files are organized by date inside this folder.">
          <Input
            value={settings.inbox_location}
            mono
            width={260}
            editable
            onChange={(v) => updateSetting("inbox_location", v)}
          />
        </Row>
        <Row label="On drop" hint="'Move' takes the original off your desktop. 'Copy' leaves it.">
          <div className="flex rounded-lg border border-border-strong bg-surface-2 p-0.5">
            <span
              className={`rounded-md px-3 py-[5px] text-xs font-semibold cursor-pointer transition-colors ${
                settings.delete_source_after_ingest
                  ? "bg-accent text-accent-fg"
                  : "text-text-muted"
              }`}
              onClick={() => updateSetting("delete_source_after_ingest", true)}
            >
              Move
            </span>
            <span
              className={`rounded-md px-3 py-[5px] text-xs cursor-pointer transition-colors ${
                !settings.delete_source_after_ingest
                  ? "bg-accent text-accent-fg"
                  : "text-text-muted"
              }`}
              onClick={() => updateSetting("delete_source_after_ingest", false)}
            >
              Copy
            </span>
          </div>
        </Row>
        <Row label="Conflict resolution" hint="When a file with the same name already exists.">
          <Select
            value={settings.conflict_resolution}
            options={CONFLICT_OPTIONS}
            onChange={(v) => updateSetting("conflict_resolution", v)}
          />
        </Row>
        <Row label="Date subfolders" hint="Group ingested files into YYYY-MM-DD folders." last>
          <Switch aria-label="Date subfolders" defaultChecked />
        </Row>
      </Section>

      <Section title="Capacity">
        <Row label="Inbox size limit" hint="Older files are flagged but not deleted.">
          <Select
            value="10_gb"
            options={[{ value: "10_gb", label: "10 GB" }]}
          />
        </Row>
        <Row label="Auto-archive" hint="Move processed files into ~/Draction/Archive after…" last>
          <Select
            value="30_days"
            options={[{ value: "30_days", label: "30 days" }]}
          />
        </Row>
      </Section>

      <Section title="Undo">
        <Row label="Undo window" hint="Time to 'oops' and restore a dropped file.">
          <Select
            value={String(settings.undo_window_seconds)}
            options={UNDO_WINDOW_OPTIONS}
            onChange={(v) => updateSetting("undo_window_seconds", Number(v))}
          />
        </Row>
        <Row label="Undo history depth" hint="Number of recent ingests Draky remembers." last>
          <Select
            value="5"
            options={[{ value: "5", label: "5 events" }]}
          />
        </Row>
      </Section>
    </>
  );
}
