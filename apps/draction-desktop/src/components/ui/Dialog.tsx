import * as RadixDialog from "@radix-ui/react-dialog";
import type { ReactNode } from "react";

export const Dialog = RadixDialog.Root;
export const DialogTrigger = RadixDialog.Trigger;
export const DialogPortal = RadixDialog.Portal;
export const DialogClose = RadixDialog.Close;

interface DialogContentProps {
  title: ReactNode;
  description?: ReactNode;
  children: ReactNode;
  /** Optional element rendered in the header next to the title (e.g. status pill). */
  headerSlot?: ReactNode;
  /** Maximum width class. Defaults to a sensible card width. */
  maxWidthClass?: string;
}

export function DialogContent({
  title,
  description,
  children,
  headerSlot,
  maxWidthClass = "max-w-lg",
}: DialogContentProps) {
  return (
    <DialogPortal>
      <RadixDialog.Overlay className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <RadixDialog.Content
        className={`fixed left-1/2 top-1/2 z-50 w-full ${maxWidthClass} -translate-x-1/2 -translate-y-1/2 rounded-xl border border-border-strong bg-surface shadow-[var(--shadow-panel)] focus:outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95`}
      >
        <div className="flex items-center justify-between border-b border-border px-6 py-4">
          <div className="flex items-center gap-3">
            <RadixDialog.Title className="font-semibold text-text">{title}</RadixDialog.Title>
            {headerSlot}
          </div>
          <RadixDialog.Close
            aria-label="Close dialog"
            className="rounded p-1 text-text-subtle leading-none text-lg transition-colors hover:bg-surface-2 hover:text-text"
          >
            ×
          </RadixDialog.Close>
        </div>
        {description && (
          <RadixDialog.Description className="px-6 pt-3 text-sm text-text-muted">
            {description}
          </RadixDialog.Description>
        )}
        <div className="p-6 space-y-4">{children}</div>
      </RadixDialog.Content>
    </DialogPortal>
  );
}
