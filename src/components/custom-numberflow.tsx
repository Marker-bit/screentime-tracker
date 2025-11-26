import { useSettingsStore } from "@/lib/settings";
import { cn } from "@/lib/utils";
import NumberFlow, { NumberFlowProps } from "@number-flow/react";

export function CustomNumberFlow({ className, ...props }: NumberFlowProps) {
  const { settings } = useSettingsStore();

  return (
    <NumberFlow
      {...props}
      animated={!settings.disableAnimation}
      className={cn(className, settings.disableAnimation && "tabular-nums")}
    />
  );
}
