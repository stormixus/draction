import * as RadixSwitch from "@radix-ui/react-switch";
import type { ComponentPropsWithoutRef } from "react";

type SwitchProps = ComponentPropsWithoutRef<typeof RadixSwitch.Root> & {
  "aria-label": string;
};

export function Switch({ className = "", ...props }: SwitchProps) {
  return (
    <RadixSwitch.Root
      {...props}
      className={
        "relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors " +
        "data-[state=unchecked]:bg-surface-2 data-[state=checked]:bg-accent " +
        "disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer " +
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus " +
        className
      }
    >
      <RadixSwitch.Thumb className="block h-3.5 w-3.5 translate-x-1 rounded-full bg-white shadow transition-transform data-[state=checked]:translate-x-4" />
    </RadixSwitch.Root>
  );
}
