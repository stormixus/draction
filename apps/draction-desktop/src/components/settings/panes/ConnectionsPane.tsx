import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Switch } from "../../ui/Switch";
import { PaneHeader } from "../PaneHeader";
import { Row } from "../Row";
import { Section } from "../Section";
import { Input } from "../Input";
import { Btn } from "../Btn";
import { Chip } from "../Chip";
import { useToast } from "../../ui/Toast";
import { setAuthToken } from "../../../lib/settings";
import { useSettingsStore } from "../../../stores/settingsStore";
import { useI18n } from "../../../lib/i18n";

function maskToken(token: string) {
  if (!token) return "No token";
  return `${token.slice(0, 6)}··········${token.slice(-4)}`;
}

export function ConnectionsPane() {
  const t = useI18n();
  const settings = useSettingsStore((s) => s.settings);
  const updateSetting = useSettingsStore((s) => s.updateSetting);
  const updateSettings = useSettingsStore((s) => s.updateSettings);
  const toast = useToast();
  const [token, setToken] = useState("");

  useEffect(() => {
    invoke<string>("get_auth_token")
      .then(setToken)
      .catch(() => setToken(""));
  }, []);

  if (!settings) return null;

  return (
    <>
      <PaneHeader title={t("Connections")} sub={t("Local services that can talk to Draction.")} />

      <Section title={t("Local API")}>
        <Row label={t("HTTP port")}>
          <Input value={String(settings.api_port)} mono width={100} />
        </Row>
        <Row label={t("Bind address")} hint={t("Localhost only — never exposed to the network.")}>
          <Input
            value={settings.api_bind_address}
            mono
            width={140}
            editable
            onChange={(v) => updateSetting("api_bind_address", v)}
          />
        </Row>
        <Row label={t("Bearer token")} hint={t("Required for OpenClaw and other clients.")} last>
          <div className="flex items-center gap-2">
            <Input value={maskToken(token)} mono width={200} />
            <Btn
              onClick={() => {
                invoke<string>("rotate_auth_token")
                  .then((next) => {
                    setToken(next);
                    setAuthToken(next);
                    toast.success(t("Bearer token rotated"));
                  })
                  .catch((err) => toast.error(t("Could not rotate token"), { description: String(err) }));
              }}
            >
              {t("Rotate")}
            </Btn>
          </div>
        </Row>
      </Section>

      <Section title={t("OpenClaw bridge")} desc={t("The optional AI peer that suggests rules.")}>
        <Row label={t("Status")}>
          <Chip tone={settings.openclaw_paired ? "accent" : "warn"}>
            {settings.openclaw_paired ? t("Connected") : t("Not connected")}
          </Chip>
        </Row>
        <Row label={t("Auto-suggest rules")} hint={t("Draky asks 'always do this?' after each ingest.")}>
          <Switch
            aria-label={t("Auto-suggest rules")}
            checked={settings.openclaw_auto_suggest_rules}
            onCheckedChange={(v) => updateSetting("openclaw_auto_suggest_rules", v)}
          />
        </Row>
        <Row label={t("Send file metadata")} hint={t("Filename, size, mime — never file contents.")} last>
          <Switch
            aria-label={t("Send file metadata")}
            checked={settings.openclaw_send_file_metadata}
            onCheckedChange={(v) => updateSetting("openclaw_send_file_metadata", v)}
          />
        </Row>
      </Section>

      <Section title={t("Pairing")}>
        <div className="flex items-center gap-[18px] p-[18px]">
          <div
            className="h-24 w-24 rounded-lg"
            style={{
              backgroundImage: "repeating-conic-gradient(#000 0 25%, #fff 0 50%)",
              backgroundSize: "12px 12px",
            }}
          />
          <div className="flex-1">
            <div className="mb-1 text-[13px] font-semibold">{t("Pair OpenClaw")}</div>
            <div className="mb-2.5 text-xs leading-relaxed text-text-muted">
              {t("Scan from OpenClaw's \"Add bridge\" screen, or enter the code manually. Pairing approval is required from this device.")}
            </div>
            <div className="flex gap-2">
              <Input
                value={settings.pairing_code}
                mono
                width={140}
                editable
                onChange={(v) => updateSetting("pairing_code", v)}
              />
              <Btn
                variant="primary"
                onClick={() =>
                  updateSettings({
                    openclaw_paired: true,
                    openclaw_device_id: settings.pairing_code || "local-openclaw",
                  })
                }
              >
                {t("Approve incoming")}
              </Btn>
            </div>
          </div>
        </div>
      </Section>
    </>
  );
}
