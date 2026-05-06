import { useState } from "react";
import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Btn } from "../Btn";
import { Input } from "../Input";
import { useSettingsStore } from "../../../stores/settingsStore";
import { translate, useI18n } from "../../../lib/i18n";

const CONFIRMATION_LABELS: Record<string, string> = {
  system_folders: "Confirm rules that touch system folders",
  batch_ingest: "Confirm batch ingests over 100 files",
};

export function SafetyPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);
  const updateSettings = useSettingsStore((s) => s.updateSettings);
  const [newScope, setNewScope] = useState("~/Draction/Work");

  if (!settings) return null;

  const paths = settings.allowed_path_scopes;
  const dangerousNodes = settings.allowed_dangerous_nodes;

  function toggleConfirmation(key: string, enabled: boolean) {
    const current = settings!.require_confirmation_for;
    const next = enabled
      ? [...current, key]
      : current.filter((k) => k !== key);
    updateSetting("require_confirmation_for", next);
  }

  function toggleDangerousNode(key: string, enabled: boolean) {
    const nextNodes = enabled
      ? Array.from(new Set([...dangerousNodes, key]))
      : dangerousNodes.filter((item) => item !== key);
    updateSettings({
      allowed_dangerous_nodes: nextNodes,
      dangerous_nodes_enabled: nextNodes.length > 0,
    });
  }

  return (
    <>
      <PaneHeader title={t("Safety")} sub={t("Limits and confirmations that protect you from oopsies.")} />

      <Section title={t("Path scopes")} desc={t("Workflows can only read or write inside these roots.")}>
        {paths.map((p, i, arr) => (
          <Row
            key={p}
            label={<span className="font-mono text-xs">{p}</span>}
            hint={t("Allowed scope")}
            last={i === arr.length - 1}
          >
            <Btn
              variant="ghost"
              onClick={() => {
                const next = paths.filter((_, idx) => idx !== i);
                updateSetting("allowed_path_scopes", next);
              }}
            >
              {t("Remove")}
            </Btn>
          </Row>
        ))}
        <div className="border-t border-border p-3">
          <div className="flex items-center gap-2">
            <Input
              value={newScope}
              mono
              width={220}
              editable
              onChange={setNewScope}
            />
            <Btn
              onClick={() => {
                const scope = newScope.trim();
                if (scope && !paths.includes(scope)) {
                  updateSetting("allowed_path_scopes", [...paths, scope]);
                }
              }}
            >
              {t("Add")}
            </Btn>
          </div>
        </div>
      </Section>

      <Section title={t("Dangerous nodes")} desc={t("Disabled by default. Each one needs a second confirmation.")}>
        <Row label={<span><code className="font-mono text-warning">exec</code> · {t("run shell commands")}</span>}>
          <Switch
            aria-label="Allow dangerous node: exec"
            checked={dangerousNodes.includes("exec")}
            onCheckedChange={(v) => toggleDangerousNode("exec", v)}
          />
        </Row>
        <Row label={<span><code className="font-mono text-warning">ssh</code> · {t("remote shell")}</span>}>
          <Switch
            aria-label="Allow ssh node"
            checked={dangerousNodes.includes("ssh")}
            onCheckedChange={(v) => toggleDangerousNode("ssh", v)}
          />
        </Row>
        <Row
          label={<span><code className="font-mono text-danger">delete</code> · {t("permanently remove files")}</span>}
          last
        >
          <Switch
            aria-label="Allow delete node"
            checked={dangerousNodes.includes("delete")}
            onCheckedChange={(v) => toggleDangerousNode("delete", v)}
          />
        </Row>
      </Section>

      <Section title={t("Confirmations")}>
        {Object.entries(CONFIRMATION_LABELS).map(([key, label]) => (
          <Row key={key} label={translate(label, settings.language)} last={key === "batch_ingest"}>
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
