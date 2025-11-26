import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";
import { PlusIcon, XIcon } from "lucide-react";
import { useEffect, useState } from "react";

export function AutoStartControlSwitch() {
  const [isAutoStartEnabled, setIsAutoStartEnabled] = useState<boolean | null>(
    null,
  );
  const [toggleLoading, setToggleLoading] = useState(false);

  useEffect(() => {
    const checkAutoStart = async () => {
      setIsAutoStartEnabled(await isEnabled());
    };

    checkAutoStart();
  }, []);

  const setAutoStart = async (on: boolean) => {
    setToggleLoading(true);
    if (!on) {
      await disable();
      setIsAutoStartEnabled(false);
    } else {
      await enable();
      setIsAutoStartEnabled(true);
    }
    setToggleLoading(false);
  };

  return (
    isAutoStartEnabled !== null && (
      <Switch
        checked={isAutoStartEnabled}
        onCheckedChange={setAutoStart}
        disabled={toggleLoading}
      />
    )
  );
}
