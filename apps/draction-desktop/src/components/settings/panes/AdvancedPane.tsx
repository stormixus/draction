import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Input } from "../Input";
import { Select } from "../Select";
import { Btn } from "../Btn";

export function AdvancedPane() {
  return (
    <>
      <PaneHeader title="Advanced" sub="For power users — most folks should leave these alone." />

      <Section title="Storage">
        <Row label="Database" hint="Events, runs and audit trail.">
          <div className="flex items-center gap-2">
            <Input value="~/Draction/draction.db" mono width={240} />
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
          <Select value="info" />
        </Row>
        <Row label="Persist logs">
          <Select value="14 days" />
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
