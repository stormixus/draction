import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { useSettingsStore } from "../../../stores/settingsStore";

const MATCH_POLICY_OPTIONS = [
  { value: "first_match", label: "First-match wins" },
  { value: "all_matches", label: "All matches" },
];

const CONCURRENCY_OPTIONS = [
  { value: "1", label: "Serial (1 worker)" },
  { value: "2", label: "Parallel (2 workers)" },
  { value: "4", label: "Parallel (4 workers)" },
];

export function RulesBehaviorPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title="Rules behavior" sub="How Draction picks and runs rules on incoming files." />

      <Section title="Matching">
        <Row label="Match policy" hint="v0.1 runs the first matching rule only.">
          <Select
            value={settings.match_policy}
            options={MATCH_POLICY_OPTIONS}
            onChange={(v) => updateSetting("match_policy", v)}
          />
        </Row>
        <Row label="Concurrency" hint="Files are processed one at a time in v0.1.">
          <Select
            value={String(settings.concurrency)}
            options={CONCURRENCY_OPTIONS}
            onChange={(v) => updateSetting("concurrency", Number(v))}
          />
        </Row>
        <Row label="Unmatched files" hint="Files that don't match any rule." last>
          <Select
            value="keep_inbox"
            options={[{ value: "keep_inbox", label: "Keep in Inbox" }]}
          />
        </Row>
      </Section>

      <Section title="Workflow execution">
        <Row label="On node failure" hint="Stop workflow at the first failed node.">
          <Select
            value="fail_fast"
            options={[{ value: "fail_fast", label: "Fail-fast (default)" }]}
          />
        </Row>
        <Row label="Auto retry" hint="Manually retry from the Runs panel.">
          <Switch aria-label="Auto retry" />
        </Row>
        <Row label="Keep partial artifacts" hint="Files produced before the failure are preserved." last>
          <Switch aria-label="Keep partial artifacts" defaultChecked />
        </Row>
      </Section>

      <Section title="Notifications">
        <Row label="Toast on success">
          <Switch aria-label="Toast on success" defaultChecked />
        </Row>
        <Row label="Toast on failure">
          <Switch aria-label="Toast on failure" defaultChecked />
        </Row>
        <Row label="Sound" last>
          <Select
            value="off"
            options={[{ value: "off", label: "Off" }]}
          />
        </Row>
      </Section>
    </>
  );
}
