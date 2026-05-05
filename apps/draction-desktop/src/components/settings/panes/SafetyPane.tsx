import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Btn } from "../Btn";

const PATHS: [string, string][] = [
  ["~/Draction/Inbox", "Default ingest target"],
  ["~/Draction/Work", "Workflow output"],
  ["~/Movies/Draction-out", "Custom — added 2026-04-19"],
];

export function SafetyPane() {
  return (
    <>
      <PaneHeader title="Safety" sub="Limits and confirmations that protect you from oopsies." />

      <Section title="Path scopes" desc="Workflows can only read or write inside these roots.">
        {PATHS.map(([p, h], i, arr) => (
          <Row
            key={p}
            label={<span className="font-mono text-xs">{p}</span>}
            hint={h}
            last={i === arr.length - 1}
          >
            <Btn variant="ghost">Remove</Btn>
          </Row>
        ))}
      </Section>

      <Section title="Dangerous nodes" desc="Disabled by default. Each one needs a second confirmation.">
        <Row label={<span><code className="font-mono text-warning">exec</code> · run shell commands</span>}>
          <Switch aria-label="Allow exec node" />
        </Row>
        <Row label={<span><code className="font-mono text-warning">ssh</code> · remote shell</span>}>
          <Switch aria-label="Allow ssh node" />
        </Row>
        <Row
          label={<span><code className="font-mono text-danger">delete</code> · permanently remove files</span>}
          last
        >
          <Switch aria-label="Allow delete node" />
        </Row>
      </Section>

      <Section title="Confirmations">
        <Row label="Confirm rules that touch system folders">
          <Switch aria-label="Confirm rules that touch system folders" defaultChecked />
        </Row>
        <Row label="Confirm batch ingests over 100 files" last>
          <Switch aria-label="Confirm batch ingests over 100 files" defaultChecked />
        </Row>
      </Section>
    </>
  );
}
