import * as RadixTabs from "@radix-ui/react-tabs";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

export const Tabs = RadixTabs.Root;
export const TabsList = RadixTabs.List;
export const TabsContent = RadixTabs.Content;

type TabsTriggerProps = ComponentPropsWithoutRef<typeof RadixTabs.Trigger> & {
  /** Optional badge (count) rendered next to the label */
  badge?: ReactNode;
};

/**
 * Pill-style trigger used by the Runs filter row. Inactive triggers fade into
 * surface; active triggers get the accent surface treatment.
 */
export function TabsTriggerPill({ children, badge, className = "", ...props }: TabsTriggerProps) {
  return (
    <RadixTabs.Trigger
      {...props}
      className={
        "flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors " +
        "text-text-subtle hover:bg-surface-2 hover:text-text-muted " +
        "data-[state=active]:bg-border-strong data-[state=active]:text-text " +
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus " +
        className
      }
    >
      {children}
      {badge !== undefined && badge !== null && (
        <span className="rounded-full bg-surface-2 px-1.5 py-0.5 text-xs text-text-subtle data-[state=active]:bg-border-strong data-[state=active]:text-text-muted">
          {badge}
        </span>
      )}
    </RadixTabs.Trigger>
  );
}

/**
 * Underline-style trigger used by the dashboard top navigation.
 */
export function TabsTriggerUnderline({ children, className = "", ...props }: TabsTriggerProps) {
  return (
    <RadixTabs.Trigger
      {...props}
      className={
        "px-4 py-3 text-sm font-medium transition-colors text-text-subtle hover:text-text-muted " +
        "data-[state=active]:border-b-2 data-[state=active]:border-accent data-[state=active]:text-accent " +
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus " +
        className
      }
    >
      {children}
    </RadixTabs.Trigger>
  );
}
