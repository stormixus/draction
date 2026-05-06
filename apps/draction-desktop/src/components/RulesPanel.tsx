import { Switch } from "./ui/Switch";
import { useRules, useToggleRule } from "../lib/query";
import { useI18n } from "../lib/i18n";

export default function RulesPanel({ baseUrl }: { baseUrl: string }) {
  const t = useI18n();
  const rulesQuery = useRules(baseUrl);
  const toggle = useToggleRule(baseUrl);
  const rules = rulesQuery.data ?? [];

  if (rulesQuery.isLoading && rules.length === 0) {
    return (
      <div className="rounded-lg border border-border p-10 text-center text-text-subtle text-sm">
        {t("Loading rules…")}
      </div>
    );
  }

  return (
    <div>
      {rulesQuery.isError && (
        <div className="mb-3 rounded-lg border border-danger/40 bg-danger/10 p-3 text-sm text-danger">
          {t("Failed to load rules:")} {String(rulesQuery.error?.message ?? "unknown error")}
        </div>
      )}

      {!rulesQuery.isLoading && rules.length === 0 && !rulesQuery.isError && (
        <div className="rounded-lg border border-border p-10 text-center text-text-subtle text-sm">
          {t("No rules configured.")}
        </div>
      )}

      {rules.length > 0 && (
        <div className="space-y-2">
          {rules.map((rule) => (
            <div
              key={rule.id}
              className="flex items-center justify-between rounded-lg border border-border bg-surface px-4 py-3 transition-colors hover:border-border-strong"
            >
              <div className="min-w-0 mr-4">
                <div className="font-medium text-text text-sm">{rule.name}</div>
                {rule.description && (
                  <div className="text-xs text-text-subtle mt-0.5">{rule.description}</div>
                )}
                <div className="text-xs text-text-subtle font-mono mt-1 truncate">{rule.id}</div>
              </div>
              <Switch
                checked={rule.enabled}
                disabled={toggle.isPending && toggle.variables?.id === rule.id}
                onCheckedChange={(enabled) => toggle.mutate({ id: rule.id, enabled })}
                aria-label={rule.enabled ? `Disable rule ${rule.name}` : `Enable rule ${rule.name}`}
              />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
