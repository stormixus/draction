import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";

export function RulesBehaviorPane() {
  return (
    <>
      <PaneHeader title="Rules behavior" sub="How Draction picks and runs rules on incoming files." />

      <Section title="Matching">
        <Row label="Match policy" hint="v0.1 runs the first matching rule only.">
          <Select value="First-match wins" />
        </Row>
        <Row label="Concurrency" hint="Files are processed one at a time in v0.1.">
          <Select value="Serial (1 worker)" />
        </Row>
        <Row label="Unmatched files" hint="Files that don't match any rule." last>
          <Select value="Keep in Inbox" />
        </Row>
      </Section>

      <Section title="Workflow execution">
        <Row label="On node failure" hint="Stop workflow at the first failed node.">
          <Select value="Fail-fast (default)" />
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
          <Select value="Off" />
        </Row>
      </Section>
    </>
  );
}
