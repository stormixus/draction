import { useReducedMotion, type Transition, type Variants } from "framer-motion";

const STANDARD_EASE: [number, number, number, number] = [0.2, 0, 0, 1];

export const transitions = {
  fast:     { duration: 0.12, ease: STANDARD_EASE } as Transition,
  base:     { duration: 0.18, ease: STANDARD_EASE } as Transition,
  slow:     { duration: 0.26, ease: STANDARD_EASE } as Transition,
  spring:   { type: "spring", stiffness: 400, damping: 25 } as Transition,
  reduced:  { duration: 0 } as Transition,
} as const;

export const dialogMotion: Variants = {
  hidden:  { opacity: 0, scale: 0.98 },
  visible: { opacity: 1, scale: 1 },
  exit:    { opacity: 0, scale: 0.98 },
};

export const toastMotion: Variants = {
  hidden:  { opacity: 0, y: 8 },
  visible: { opacity: 1, y: 0 },
  exit:    { opacity: 0, y: 4 },
};

export const overlayMotion: Variants = {
  hidden:  { opacity: 0 },
  visible: { opacity: 1 },
  exit:    { opacity: 0 },
};

/**
 * Returns a transition that collapses to zero-duration when the user has
 * `prefers-reduced-motion: reduce`. Use as `<motion.div transition={useMotionTransition("base")}>`.
 */
export function useMotionTransition(preset: keyof typeof transitions = "base"): Transition {
  const reduce = useReducedMotion();
  return reduce ? transitions.reduced : transitions[preset];
}

/**
 * For variants: returns the variant set unchanged, but caller should pair with
 * `useMotionTransition(...)` so framer-motion respects prefers-reduced-motion.
 */
export function useMotionVariants<V extends Variants>(variants: V): V {
  return variants;
}
