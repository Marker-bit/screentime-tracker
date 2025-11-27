import { AutoStartControlSwitch } from "@/components/autostart-control";
import { ChangePasswordButton } from "@/components/change-password-button";
import { ScreenshotFolder } from "@/components/screenshot-folder";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { t } from "@/intl";
import { useSettingsStore } from "@/lib/settings";
import {
  ArrowUp10Icon,
  GlobeIcon,
  KeyIcon,
  LayoutGridIcon,
  LockIcon,
  MailIcon,
  MonitorIcon,
  PcCase,
  PlayIcon,
} from "lucide-react";

export function Settings({ onLock }: { onLock: () => void }) {
  const { settings, setSettings } = useSettingsStore();

  return (
    <div className="flex flex-col gap-2 min-w-100">
      <Button
        size="icon"
        variant="ghost"
        className="fixed top-2 right-2"
        onClick={onLock}
      >
        <LockIcon />
      </Button>
      <div className="text-muted-foreground flex gap-2 items-center">
        <MonitorIcon className="size-4" />
        {t("interface")}
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <ArrowUp10Icon className="size-4" />
          {t("disableNumberAnimation")}
        </div>
        <Switch
          checked={settings.disableAnimation}
          onCheckedChange={(value) => {
            setSettings({ ...settings, disableAnimation: value });
          }}
        />
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex flex-col gap-2">
        <div className="font-semibold flex gap-2 items-center">
          <GlobeIcon className="size-4" />
          {t("appLanguage")}
        </div>
        <Select
          value={settings.language}
          onValueChange={(value) => {
            setSettings({ ...settings, language: value });
          }}
        >
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder={t("chooseLanguage")} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="ru">Русский</SelectItem>
            <SelectItem value="en">English</SelectItem>
          </SelectContent>
        </Select>
      </div>
      <div className="text-muted-foreground flex gap-2 items-center">
        <PcCase className="size-4" />
        {t("system")}
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <PlayIcon className="size-4" />
          {t("autostart")}
        </div>
        <AutoStartControlSwitch />
      </div>
      <div className="text-muted-foreground flex gap-2 items-center">
        <LayoutGridIcon className="size-4" />
        {t("app")}
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <KeyIcon className="size-4" /> {t("settingsPassword")}
        </div>
        <ChangePasswordButton
          buttonText={t("changePassword")}
          size="sm"
        ></ChangePasswordButton>
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex flex-col gap-2">
        <div className="font-semibold flex gap-2 items-center">
          <MailIcon className="size-4" />
          {t("parentEmail")}
        </div>
        <Input
          className="font-mono w-48"
          value={settings.parentEmail}
          onChange={(evt) => {
            setSettings({ ...settings, parentEmail: evt.target.value });
          }}
        />
      </div>
      <ScreenshotFolder />
    </div>
  );
}
