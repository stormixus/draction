import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { useSettingsStore } from "../../../stores/settingsStore";
import { trOptions, useI18n } from "../../../lib/i18n";

const MATCH_POLICY_OPTIONS = [
  { value: "first_match", label: "First-match wins" },
  { value: "all_matches", label: "All matches" },
];

const CONCURRENCY_OPTIONS = [
  { value: "1", label: "Serial (1 worker)" },
  { value: "2", label: "Parallel (2 workers)" },
  { value: "4", label: "Parallel (4 workers)" },
];

const UNMATCHED_OPTIONS = [
  { value: "keep_inbox", label: "Keep in Inbox" },
  { value: "flag_only", label: "Flag only" },
  { value: "archive", label: "Archive" },
];

const FAILURE_OPTIONS = [
  { value: "fail_fast", label: "Fail-fast (default)" },
  { value: "continue", label: "Continue workflow" },
];

const SOUND_OPTIONS = [
  { value: "off", label: "Off" },
  { value: "soft", label: "Soft chime" },
  { value: "draky", label: "Draky chirp" },
];

export function RulesBehaviorPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title={t("Rules behavior")} sub={t("How Draction picks and runs rules on incoming files.")} />

      <Section title={t("Matching")}>
        <Row label={t("Match policy")} hint={t("v0.1 runs the first matching rule only.")}>
          <Select
            value={settings.match_policy}
            options={trOptions(MATCH_POLICY_OPTIONS, t)}
            onChange={(v) => updateSetting("match_policy", v)}
          />
        </Row>
        <Row label={t("Concurrency")} hint={t("Files are processed one at a time in v0.1.")}>
          <Select
            value={String(settings.concurrency)}
            options={trOptions(CONCURRENCY_OPTIONS, t)}
            onChange={(v) => updateSetting("concurrency", Number(v))}
          />
        </Row>
        <Row label={t("Unmatched files")} hint={t("Files that don't match any rule.")} last>
          <Select
            value={settings.unmatched_files_policy}
            options={trOptions(UNMATCHED_OPTIONS, t)}
            onChange={(v) => updateSetting("unmatched_files_policy", v)}
          />
        </Row>
      </Section>

      <Section title={t("Workflow execution")}>
        <Row label={t("On node failure")} hint={t("Stop workflow at the first failed node.")}>
          <Select
            value={settings.node_failure_policy}
            options={trOptions(FAILURE_OPTIONS, t)}
            onChange={(v) => updateSetting("node_failure_policy", v)}
          />
        </Row>
        <Row label={t("Auto retry")} hint={t("Manually retry from the Runs panel.")}>
          <Switch
            aria-label={t("Auto retry")}
            checked={settings.auto_retry}
            onCheckedChange={(v) => updateSetting("auto_retry", v)}
          />
        </Row>
        <Row label={t("Keep partial artifacts")} hint={t("Files produced before the failure are preserved.")} last>
          <Switch
            aria-label={t("Keep partial artifacts")}
            checked={settings.keep_partial_artifacts}
            onCheckedChange={(v) => updateSetting("keep_partial_artifacts", v)}
          />
        </Row>
      </Section>

      <Section title={t("Notifications")}>
        <Row label={t("Toast on success")}>
          <Switch
            aria-label={t("Toast on success")}
            checked={settings.toast_on_success}
            onCheckedChange={(v) => updateSetting("toast_on_success", v)}
          />
        </Row>
        <Row label={t("Toast on failure")}>
          <Switch
            aria-label={t("Toast on failure")}
            checked={settings.toast_on_failure}
            onCheckedChange={(v) => updateSetting("toast_on_failure", v)}
          />
        </Row>
        <Row label={t("Sound")} last>
          <Select
            value={settings.notification_sound}
            options={trOptions(SOUND_OPTIONS, t)}
            onChange={(v) => updateSetting("notification_sound", v)}
          />
        </Row>
      </Section>
    </>
  );
}
