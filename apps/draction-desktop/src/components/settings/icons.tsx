import type { ReactNode } from "react";

function IconWrapper({ children }: { children: ReactNode }) {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
    >
      {children}
    </svg>
  );
}

export function GeneralIcon() {
  return (
    <IconWrapper>
      <circle cx="12" cy="12" r="3" />
      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
    </IconWrapper>
  );
}

export function InboxIcon() {
  return (
    <IconWrapper>
      <path d="M22 12h-6l-2 3h-4l-2-3H2" />
      <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
    </IconWrapper>
  );
}

export function RulesIcon() {
  return (
    <IconWrapper>
      <path d="M3 6h18M7 12h14M11 18h10" />
    </IconWrapper>
  );
}

export function DrakyIcon() {
  return (
    <IconWrapper>
      <path d="M12 3c4.5 0 8 3 8 7 0 3-2 5-4 6l1 4-3-2c-.7.1-1.4.2-2 .2-4.5 0-8-3-8-7s3.5-8.2 8-8.2z" />
      <circle cx="9.5" cy="10.5" r="0.6" fill="currentColor" />
      <circle cx="14.5" cy="10.5" r="0.6" fill="currentColor" />
    </IconWrapper>
  );
}

export function LinkIcon() {
  return (
    <IconWrapper>
      <path d="M10 13a5 5 0 0 0 7.07 0l3-3a5 5 0 0 0-7.07-7.07l-1.5 1.5" />
      <path d="M14 11a5 5 0 0 0-7.07 0l-3 3a5 5 0 0 0 7.07 7.07l1.5-1.5" />
    </IconWrapper>
  );
}

export function ShieldIcon() {
  return (
    <IconWrapper>
      <path d="M12 2 4 6v6c0 5 3.5 9.5 8 10 4.5-.5 8-5 8-10V6l-8-4z" />
    </IconWrapper>
  );
}

export function BeakerIcon() {
  return (
    <IconWrapper>
      <path d="M9 3h6M10 3v6L4 19a2 2 0 0 0 1.7 3h12.6a2 2 0 0 0 1.7-3L14 9V3" />
    </IconWrapper>
  );
}

export function AboutIcon() {
  return (
    <IconWrapper>
      <circle cx="12" cy="12" r="10" />
      <path d="M12 16v-4M12 8h.01" />
    </IconWrapper>
  );
}

export function FolderIcon() {
  return (
    <svg
      width="14"
      height="14"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
    >
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
    </svg>
  );
}

export function ResetIcon() {
  return (
    <svg
      width="14"
      height="14"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
    >
      <path d="M3 12a9 9 0 1 0 9-9c-2.5 0-4.7 1-6.4 2.6L3 8" />
      <path d="M3 3v5h5" />
    </svg>
  );
}
