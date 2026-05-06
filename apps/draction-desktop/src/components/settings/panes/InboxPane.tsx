import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { Input } from "../Input";
import { useSettingsStore } from "../../../stores/settingsStore";
import { trOptions, useI18n } from "../../../lib/i18n";

const CONFLICT_OPTIONS = [
  { value: "append_timestamp", label: "Append timestamp" },
  { value: "rename", label: "Rename with counter" },
  { value: "overwrite", label: "Overwrite" },
  { value: "skip", label: "Skip" },
];

const SIZE_LIMIT_OPTIONS = [
  { value: "5", label: "5 GB" },
  { value: "10", label: "10 GB" },
  { value: "25", label: "25 GB" },
  { value: "50", label: "50 GB" },
];

const ARCHIVE_OPTIONS = [
  { value: "7", label: "7 days" },
  { value: "30", label: "30 days" },
  { value: "90", label: "90 days" },
  { value: "0", label: "Off" },
];

const UNDO_WINDOW_OPTIONS = [
  { value: "10", label: "10 seconds" },
  { value: "30", label: "30 seconds" },
  { value: "60", label: "60 seconds" },
  { value: "120", label: "2 minutes" },
];

const UNDO_DEPTH_OPTIONS = [
  { value: "3", label: "3 events" },
  { value: "5", label: "5 events" },
  { value: "10", label: "10 events" },
  { value: "25", label: "25 events" },
];

export function InboxPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title={t("Inbox")} sub={t("Where dropped files land before workflows pick them up.")} />

      <Section title={t("Inbox folder")}>
        <Row label={t("Location")} hint={t("Files are organized by date inside this folder.")}>
          <Input
            value={settings.inbox_location}
            mono
            width={260}
            editable
            onChange={(v) => updateSetting("inbox_location", v)}
          />
        </Row>
        <Row label={t("On drop")} hint={t("'Move' takes the original off your desktop. 'Copy' leaves it.")}>
          <div className="flex rounded-lg border border-border-strong bg-surface-2 p-0.5">
            <span
              className={`rounded-md px-3 py-[5px] text-xs font-semibold cursor-pointer transition-colors ${
                settings.delete_source_after_ingest
                  ? "bg-accent text-accent-fg"
                  : "text-text-muted"
              }`}
              onClick={() => updateSetting("delete_source_after_ingest", true)}
            >
              {t("Move")}
            </span>
            <span
              className={`rounded-md px-3 py-[5px] text-xs cursor-pointer transition-colors ${
                !settings.delete_source_after_ingest
                  ? "bg-accent text-accent-fg"
                  : "text-text-muted"
              }`}
              onClick={() => updateSetting("delete_source_after_ingest", false)}
            >
              {t("Copy")}
            </span>
          </div>
        </Row>
        <Row label={t("Conflict resolution")} hint={t("When a file with the same name already exists.")}>
          <Select
            value={settings.conflict_resolution}
            options={trOptions(CONFLICT_OPTIONS, t)}
            onChange={(v) => updateSetting("conflict_resolution", v)}
          />
        </Row>
        <Row label={t("Date subfolders")} hint={t("Group ingested files into YYYY-MM-DD folders.")} last>
          <Switch
            aria-label={t("Date subfolders")}
            checked={settings.date_subfolders}
            onCheckedChange={(v) => updateSetting("date_subfolders", v)}
          />
        </Row>
      </Section>

      <Section title={t("Capacity")}>
        <Row label={t("Inbox size limit")} hint={t("Older files are flagged but not deleted.")}>
          <Select
            value={String(settings.inbox_size_limit_gb)}
            options={SIZE_LIMIT_OPTIONS}
            onChange={(v) => updateSetting("inbox_size_limit_gb", Number(v))}
          />
        </Row>
        <Row label={t("Auto-archive")} hint={t("Move processed files into ~/Draction/Archive after…")} last>
          <Select
            value={String(settings.auto_archive_days)}
            options={ARCHIVE_OPTIONS}
            onChange={(v) => updateSetting("auto_archive_days", Number(v))}
          />
        </Row>
      </Section>

      <Section title={t("Undo")}>
        <Row label={t("Undo window")} hint={t("Time to 'oops' and restore a dropped file.")}>
          <Select
            value={String(settings.undo_window_seconds)}
            options={UNDO_WINDOW_OPTIONS}
            onChange={(v) => updateSetting("undo_window_seconds", Number(v))}
          />
        </Row>
        <Row label={t("Undo history depth")} hint={t("Number of recent ingests Draky remembers.")} last>
          <Select
            value={String(settings.undo_history_depth)}
            options={UNDO_DEPTH_OPTIONS}
            onChange={(v) => updateSetting("undo_history_depth", Number(v))}
          />
        </Row>
      </Section>
    </>
  );
}
