import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
} from "@/components/ui/input-otp";
import { t } from "@/intl";
import { useSettingsStore } from "@/lib/settings";
import { cn } from "@/lib/utils";
import { REGEXP_ONLY_DIGITS } from "input-otp";
import { CircleCheck } from "lucide-react";
import { ComponentPropsWithoutRef, useState } from "react";
import { toast } from "sonner";

export function ChangePasswordButton({
  buttonText,
  ...props
}: ComponentPropsWithoutRef<typeof Button> & { buttonText: string }) {
  const [step, setStep] = useState<
    "prev-password" | "password" | "confirm" | "success"
  >("prev-password");
  const [inputValue, setInputValue] = useState("");
  const [setPassword, setSetPassword] = useState<string>("");
  const [confirmPassword, setConfirmPassword] = useState<string>("");
  const [open, _setOpen] = useState(false);
  const { settings, setSettings } = useSettingsStore();

  const setOpen = (val: boolean) => {
    if (val === false) {
      setStep("prev-password");
      setInputValue("");
      setSetPassword("");
      setConfirmPassword("");
    }
    _setOpen(val);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button {...props}>{buttonText}</Button>
      </DialogTrigger>
      <DialogContent>
        {step !== "success" ? (
          <div className="flex flex-col gap-4 items-center justify-center size-full">
            <div className="flex gap-2 items-center">
              <div
                className={cn(
                  "size-2 rounded-full transition-colors",
                  step === "prev-password" ? "bg-primary" : "bg-primary/10",
                )}
              />
              <div
                className={cn(
                  "size-2 rounded-full transition-colors",
                  step === "password" ? "bg-primary" : "bg-primary/10",
                )}
              />
              <div
                className={cn(
                  "size-2 rounded-full transition-colors",
                  step === "confirm" ? "bg-primary" : "bg-primary/10",
                )}
              />
            </div>
            <div className="flex flex-col gap-2 items-center text-center">
              <h1 className="font-semibold text-3xl">
                {t(
                  step === "prev-password"
                    ? "enterPreviousPassword"
                    : step === "password"
                      ? "enterNewPassword"
                      : "confirmNewPassword",
                )}
              </h1>
              <p className="text-sm text-muted-foreground max-w-[60%]">
                {t(
                  step === "prev-password"
                    ? "enterPreviousPasswordDesc"
                    : step === "password"
                      ? "enterNewPasswordDesc"
                      : "confirmNewPasswordDesc",
                )}
              </p>
            </div>
            <InputOTP
              maxLength={4}
              value={
                step === "prev-password"
                  ? inputValue
                  : step === "password"
                    ? setPassword
                    : confirmPassword
              }
              onChange={
                step === "prev-password"
                  ? setInputValue
                  : step === "password"
                    ? setSetPassword
                    : setConfirmPassword
              }
              pattern={REGEXP_ONLY_DIGITS}
              onComplete={(val: string) => {
                if (step === "password") {
                  setStep("confirm");
                  return;
                }
                if (step === "confirm") {
                  if (setPassword !== confirmPassword) {
                    toast.error(t("passwordsDoNotMatch"));
                    setConfirmPassword("");
                    return;
                  }
                  setStep("success");
                  setSettings({ ...settings, password: setPassword });
                  return;
                }
                if (val === settings.password) {
                  setStep("password");
                  setSetPassword("");
                } else {
                  setInputValue("");
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
          </div>
        ) : (
          <div className="min-h-60 w-full flex flex-col text-center gap-2 items-center justify-center">
            <CircleCheck className="size-8 shrink-0" />
            <h1 className="font-semibold text-3xl">{t("newPasswordSet")}</h1>
            <p className="text-sm text-muted-foreground max-w-[60%]">
              {t("newPasswordSetDescription")}
            </p>
            <Button
              className="mt-4"
              onClick={() => {
                setOpen(false);
              }}
            >
              {t("close")}
            </Button>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
