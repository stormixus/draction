import { invoke } from "@tauri-apps/api/core";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Input } from "../Input";
import { Select } from "../Select";
import { Btn } from "../Btn";
import { useToast } from "../../ui/Toast";
import { useSettingsStore } from "../../../stores/settingsStore";
import { useI18n } from "../../../lib/i18n";

const LOG_LEVEL_OPTIONS = [
  { value: "trace", label: "trace" },
  { value: "debug", label: "debug" },
  { value: "info", label: "info" },
  { value: "warn", label: "warn" },
  { value: "error", label: "error" },
];

const LOG_RETENTION_OPTIONS = [
  { value: "7", label: "7 days" },
  { value: "14", label: "14 days" },
  { value: "30", label: "30 days" },
  { value: "90", label: "90 days" },
];

export function AdvancedPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);
  const loadSettings = useSettingsStore((s) => s.loadSettings);
  const toast = useToast();

  if (!settings) return null;

  function runCommand(command: string, success: string, args?: Record<string, unknown>) {
    invoke(command, args)
      .then(() => {
        toast.success(success);
        return loadSettings();
      })
      .catch((err) => toast.error(t("Command failed"), { description: String(err) }));
  }

  return (
    <>
      <PaneHeader title={t("Advanced")} sub={t("For power users — most folks should leave these alone.")} />

      <Section title={t("Storage")}>
        <Row label={t("Database")} hint={t("Events, runs and audit trail.")}>
          <div className="flex items-center gap-2">
            <Input value={settings.db_path} mono width={240} />
            <Btn onClick={() => runCommand("reveal_path", t("Database revealed"), { path: settings.db_path })}>
              {t("Reveal")}
            </Btn>
          </div>
        </Row>
        <Row label={t("Rules file")}>
          <Input value={settings.rules_path} mono width={240} />
        </Row>
        <Row label={t("Workflows file")} last>
          <Input value={settings.workflows_path} mono width={240} />
        </Row>
      </Section>

      <Section title={t("Logs")}>
        <Row label={t("Log level")}>
          <Select
            value={settings.log_level}
            options={LOG_LEVEL_OPTIONS}
            onChange={(v) => updateSetting("log_level", v)}
          />
        </Row>
        <Row label={t("Persist logs")}>
          <Select
            value={String(settings.log_retention_days)}
            options={LOG_RETENTION_OPTIONS}
            onChange={(v) => updateSetting("log_retention_days", Number(v))}
          />
        </Row>
        <Row label={t("Open log folder")} last>
          <Btn onClick={() => runCommand("open_path", t("Log folder opened"), { path: "~/Draction/logs" })}>
            {t("Open")}
          </Btn>
        </Row>
      </Section>

      <Section title={t("Reset")}>
        <Row label={t("Reset to defaults")} hint={t("Restores General, Inbox and Draky panes only.")}>
          <Btn
            onClick={() =>
              invoke("reset_settings_section", { section: "general" })
                .then(() => invoke("reset_settings_section", { section: "inbox" }))
                .then(() => invoke("reset_settings_section", { section: "draky" }))
                .then(() => {
                  toast.success(t("Settings reset"));
                  return loadSettings();
                })
                .catch((err) => toast.error(t("Reset failed"), { description: String(err) }))
            }
          >
            {t("Reset")}
          </Btn>
        </Row>
        <Row label={t("Clear database")} hint={t("Removes all events, runs and undo history.")}>
          <Btn
            danger
            onClick={() => {
              if (window.confirm(t("Clear all events, runs and undo history?"))) {
                runCommand("clear_database", t("Database history cleared"));
              }
            }}
          >
            {t("Clear…")}
          </Btn>
        </Row>
        <Row label={t("Erase Draction")} hint={t("Deletes ~/Draction entirely. Cannot be undone.")} last>
          <Btn
            danger
            onClick={() => {
              if (window.confirm(t("Erase settings, rules, workflows and runtime folders? This cannot be undone."))) {
                runCommand("erase_runtime_data", t("Runtime data erased"));
              }
            }}
          >
            {t("Erase…")}
          </Btn>
        </Row>
      </Section>
    </>
  );
}
