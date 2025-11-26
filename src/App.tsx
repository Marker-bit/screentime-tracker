import { CustomNumberFlow } from "@/components/custom-numberflow";
import { Settings } from "@/components/settings";
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
} from "@/components/ui/input-otp";
import { toast } from "sonner";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSettingsStore } from "@/lib/settings";
import { listen } from "@tauri-apps/api/event";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import { AnimatePresence, motion } from "motion/react";
import { useState } from "react";

type TotalTimeEvent = number;

function App() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [totalTime, setTotalTime] = useState(0);
  const [appsInfo, setAppsInfo] = useState<object | null>(null);
  const [settingsPassword, setSettingsPassword] = useState("");
  const { settings, setSettings } = useSettingsStore();

  listen<TotalTimeEvent>("total-time", (event) => {
    setTotalTime(event.payload);
  });
  listen<Record<string, number>>("apps-info", (event) => {
    setAppsInfo(event.payload);
  });
  listen<undefined>("lock", (event) => {
    setSettingsOpen(false);
  });

  if (!settings.password) {
    return (
      <main className="flex flex-col gap-4 items-center overflow-hidden justify-center h-dvh">
        <div className="flex flex-col gap-2 items-center text-center">
          <h1 className="font-semibold text-3xl">Придумайте пароль</h1>
          <p className="text-sm text-muted-foreground max-w-[60%]">
            Для доступа к настройкам требуется придумать пароль из четырёх цифр
          </p>
        </div>
        <InputOTP
          maxLength={4}
          pattern={REGEXP_ONLY_DIGITS}
          onComplete={(val: string) => {
            setSettings({ ...settings, password: val });
          }}
        >
          <InputOTPGroup>
            <InputOTPSlot index={0} />
            <InputOTPSlot index={1} />
            <InputOTPSlot index={2} />
            <InputOTPSlot index={3} />
          </InputOTPGroup>
        </InputOTP>
      </main>
    );
  }

  return (
    <main className="flex flex-col items-center overflow-hidden justify-center h-dvh">
      <Tabs className="items-center" defaultValue="total-time">
        <TabsList className="gap-1 bg-transparent fixed top-2 left-1/2 -translate-x-1/2 z-50">
          <TabsTrigger
            className="rounded-full data-[state=active]:bg-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-none transition-none"
            value="total-time"
          >
            Всё время
          </TabsTrigger>
          <TabsTrigger
            className="rounded-full data-[state=active]:bg-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-none transition-none"
            value="app-info"
          >
            Время по приложениям
          </TabsTrigger>
          <TabsTrigger
            className="rounded-full data-[state=active]:bg-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-none transition-none"
            value="settings"
          >
            Настройки
          </TabsTrigger>
        </TabsList>
        <TabsContent
          value="total-time"
          className="flex flex-col items-center justify-center"
        >
          {/*<Button onClick={() => invoke("greet", { name: "Mark" })}>
            Отправить письмо
          </Button>*/}
          <div className="text-muted-foreground">Время использования</div>
          <CustomNumberFlow
            className="font-black text-5xl"
            suffix=" мин"
            value={totalTime / 1000 / 60}
            format={{
              maximumFractionDigits: 2,
              minimumFractionDigits: 0,
            }}
          ></CustomNumberFlow>
        </TabsContent>
        <TabsContent value="app-info">
          <div className="flex flex-col min-w-80 max-h-[80vh] overflow-y-auto overflow-x-hidden">
            <AnimatePresence initial={false}>
              {appsInfo &&
                Object.entries(appsInfo).map(([appName, appTime]) => (
                  <motion.div
                    className="rounded-xl bg-secondary text-secondary-foreground flex items-center gap-2 justify-between"
                    initial={{
                      opacity: 0,
                      paddingTop: "0",
                      paddingBottom: "0",
                      paddingLeft: "0",
                      paddingRight: "0",
                      scaleX: 0.7,
                      marginBottom: "0",
                      filter: "blur(10px)",
                    }}
                    animate={{
                      opacity: 1,
                      paddingTop: "0.5rem",
                      paddingBottom: "0.5rem",
                      paddingLeft: "1rem",
                      paddingRight: "1rem",
                      scaleX: 1,
                      marginBottom: "1rem",
                      filter: "blur(0px)",
                    }}
                    exit={{
                      opacity: 0,
                      paddingTop: "0",
                      paddingBottom: "0",
                      paddingLeft: "0",
                      paddingRight: "0",
                      scaleX: 0.7,
                      marginBottom: "0",
                      filter: "blur(10px)",
                    }}
                    key={appName}
                  >
                    <div className="font-semibold">{appName}</div>
                    <CustomNumberFlow
                      className="text-sm text-muted-foreground"
                      suffix=" мин"
                      value={appTime / 1000 / 60}
                      format={{
                        maximumFractionDigits: 2,
                        minimumFractionDigits: 0,
                      }}
                    ></CustomNumberFlow>
                  </motion.div>
                ))}
            </AnimatePresence>
          </div>
        </TabsContent>
        <TabsContent
          value="settings"
          className="flex flex-col items-center justify-center gap-4"
        >
          {settingsOpen ? (
            <Settings onLock={() => setSettingsOpen(false)} />
          ) : (
            <>
              <div className="flex flex-col gap-2 items-center text-center">
                <h1 className="font-semibold text-3xl">Введите пароль</h1>
                <p className="text-sm text-muted-foreground max-w-[60%]">
                  Для доступа к настройкам требуется ввести установленный при
                  первом запуске пароль
                </p>
              </div>
              <InputOTP
                maxLength={4}
                pattern={REGEXP_ONLY_DIGITS}
                value={settingsPassword}
                onChange={setSettingsPassword}
                onComplete={(val: string) => {
                  if (val === settings.password) {
                    setSettingsOpen(true);
                  } else {
                    toast.error("Пароль неверный");
                    setSettingsPassword("");
                  }
                }}
              >
                <InputOTPGroup>
                  <InputOTPSlot index={0} />
                  <InputOTPSlot index={1} />
                  <InputOTPSlot index={2} />
                  <InputOTPSlot index={3} />
                </InputOTPGroup>
              </InputOTP>
            </>
          )}
        </TabsContent>
      </Tabs>
    </main>
  );
}

export default App;
