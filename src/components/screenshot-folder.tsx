import { openPath } from "@tauri-apps/plugin-opener";

import { Button } from "@/components/ui/button";
import { t } from "@/intl";
import { FolderIcon, Loader2Icon, LoaderIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

export function ScreenshotFolder() {
  const [screenshotFolder, setScreenshotFolder] = useState<string | null>(null);

  useEffect(() => {
    invoke<string>("get_screenshots_dir")
      .then((f) => setScreenshotFolder(f))
      .catch((e: string) => toast.error(e));
  }, []);

  return (
    <div className="rounded-xl bg-secondary text-secondary-foreground px-4 py-2 flex items-center gap-2 justify-between">
      <div className="flex flex-col gap-2">
        <div className="font-semibold flex gap-2 items-center">
          <FolderIcon className="size-4" />
          {t("screenshotsFolder")}
        </div>
        <div className="text-muted-foreground text-xs truncate max-w-60">
          {screenshotFolder ?? <Loader2Icon className="size-4 animate-spin" />}
        </div>
      </div>
      <Button
        size="sm"
        onClick={() => screenshotFolder && openPath(screenshotFolder)}
        disabled={!screenshotFolder}
      >
        Открыть
      </Button>
    </div>
  );
}
