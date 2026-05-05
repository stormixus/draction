import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Select } from "../Select";
import { Input } from "../Input";

export function InboxPane() {
  return (
    <>
      <PaneHeader title="Inbox" sub="Where dropped files land before workflows pick them up." />

      <Section title="Inbox folder">
        <Row label="Location" hint="Files are organized by date inside this folder.">
          <Input value="~/Draction/Inbox" mono width={260} />
        </Row>
        <Row label="On drop" hint="'Move' takes the original off your desktop. 'Copy' leaves it.">
          <div className="flex rounded-lg border border-border-strong bg-surface-2 p-0.5">
            <span className="rounded-md bg-accent px-3 py-[5px] text-xs font-semibold text-accent-fg">Move</span>
            <span className="rounded-md px-3 py-[5px] text-xs text-text-muted">Copy</span>
          </div>
        </Row>
        <Row label="Conflict resolution" hint="When a file with the same name already exists.">
          <Select value="Append timestamp" />
        </Row>
        <Row label="Date subfolders" hint="Group ingested files into YYYY-MM-DD folders." last>
          <Switch aria-label="Date subfolders" defaultChecked />
        </Row>
      </Section>

      <Section title="Capacity">
        <Row label="Inbox size limit" hint="Older files are flagged but not deleted.">
          <Select value="10 GB" />
        </Row>
        <Row label="Auto-archive" hint="Move processed files into ~/Draction/Archive after…" last>
          <Select value="30 days" />
        </Row>
      </Section>

      <Section title="Undo">
        <Row label="Undo window" hint="Time to 'oops' and restore a dropped file.">
          <Select value="10 seconds" />
        </Row>
        <Row label="Undo history depth" hint="Number of recent ingests Draky remembers." last>
          <Select value="5 events" />
        </Row>
      </Section>
    </>
  );
}
