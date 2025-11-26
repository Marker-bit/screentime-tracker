import { AutoStartControlSwitch } from "@/components/autostart-control";
import { ChangePasswordButton } from "@/components/change-password-button";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { useSettingsStore } from "@/lib/settings";
import {
  ArrowUp10Icon,
  KeyIcon,
  LayoutGrid,
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
        Интерфейс
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <ArrowUp10Icon className="size-4" />
          Отключить анимацию чисел
        </div>
        <Switch
          checked={settings.disableAnimation}
          onCheckedChange={(value) => {
            console.log(value);
            setSettings({ ...settings, disableAnimation: value });
          }}
        />
      </div>
      <div className="text-muted-foreground flex gap-2 items-center">
        <PcCase className="size-4" />
        Система
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <PlayIcon className="size-4" />
          Включаться при запуске системы
        </div>
        <AutoStartControlSwitch />
      </div>
      <div className="text-muted-foreground flex gap-2 items-center">
        <LayoutGridIcon className="size-4" />
        Приложение
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
        <div className="font-semibold flex gap-2 items-center">
          <KeyIcon className="size-4" /> Пароль настроек
        </div>
        <ChangePasswordButton
          buttonText="Сменить пароль"
          size="sm"
        ></ChangePasswordButton>
      </div>
      <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex flex-col gap-2">
        <div className="font-semibold flex gap-2 items-center">
          <MailIcon className="size-4" />
          Электронный адрес родителя
        </div>
        <Input
          className="font-mono w-48"
          value={settings.parentEmail}
          onChange={(evt) => {
            setSettings({ ...settings, parentEmail: evt.target.value });
          }}
        />
      </div>
    </div>
  );
}
