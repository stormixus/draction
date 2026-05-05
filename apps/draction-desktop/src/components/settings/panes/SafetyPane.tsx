import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Btn } from "../Btn";
import { useSettingsStore } from "../../../stores/settingsStore";

const CONFIRMATION_LABELS: Record<string, string> = {
  system_folders: "Confirm rules that touch system folders",
  batch_ingest: "Confirm batch ingests over 100 files",
};

export function SafetyPane() {
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);

  if (!settings) return null;

  const paths = settings.allowed_path_scopes;

  function toggleConfirmation(key: string, enabled: boolean) {
    const current = settings!.require_confirmation_for;
    const next = enabled
      ? [...current, key]
      : current.filter((k) => k !== key);
    updateSetting("require_confirmation_for", next);
  }

  return (
    <>
      <PaneHeader title="Safety" sub="Limits and confirmations that protect you from oopsies." />

      <Section title="Path scopes" desc="Workflows can only read or write inside these roots.">
        {paths.map((p, i, arr) => (
          <Row
            key={p}
            label={<span className="font-mono text-xs">{p}</span>}
            hint="Allowed scope"
            last={i === arr.length - 1}
          >
            <Btn
              variant="ghost"
              onClick={() => {
                const next = paths.filter((_, idx) => idx !== i);
                updateSetting("allowed_path_scopes", next);
              }}
            >
              Remove
            </Btn>
          </Row>
        ))}
      </Section>

      <Section title="Dangerous nodes" desc="Disabled by default. Each one needs a second confirmation.">
        <Row label={<span><code className="font-mono text-warning">exec</code> · run shell commands</span>}>
          <Switch
            aria-label="Allow dangerous node: exec"
            checked={settings.dangerous_nodes_enabled}
            onCheckedChange={(v) => updateSetting("dangerous_nodes_enabled", v)}
          />
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
        {Object.entries(CONFIRMATION_LABELS).map(([key, label]) => (
          <Row key={key} label={label} last={key === "batch_ingest"}>
            <Switch
              aria-label={label}
              checked={settings.require_confirmation_for.includes(key)}
              onCheckedChange={(v) => toggleConfirmation(key, v)}
            />
          </Row>
        ))}
      </Section>
    </>
  );
}
