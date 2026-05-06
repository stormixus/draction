import { useEffect, useState } from "react";

const COLS = 8;
const ROWS = 8;

const FRAMES = {
  idle: [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
  wave: [[0, 5], [0, 6], [0, 7]],
  burp: [[7, 3], [7, 4], [7, 5]],
} as const;

type DrakyState = keyof typeof FRAMES;

function pingPong(frames: readonly (readonly [number, number])[]) {
  if (frames.length <= 2) return [...frames];
  return [...frames, ...frames.slice(1, -1).reverse()];
}

interface DrakySpriteProps {
  state?: DrakyState;
  size: number;
  paused?: boolean;
}

export function DrakySprite({ state = "idle", size, paused }: DrakySpriteProps) {
  const frames = pingPong(FRAMES[state] ?? FRAMES.idle);
  const [index, setIndex] = useState(0);

  useEffect(() => setIndex(0), [state]);

  useEffect(() => {
    if (paused || frames.length <= 1) return;
    const id = window.setTimeout(() => {
      setIndex((current) => (current + 1) % frames.length);
    }, state === "burp" ? 200 : 140);
    return () => window.clearTimeout(id);
  }, [frames.length, index, paused, state]);

  const [row, col] = frames[index] ?? frames[0];
  const x = (col / (COLS - 1)) * 100;
  const y = (row / (ROWS - 1)) * 100;

  return (
    <div
      className="shrink-0"
      style={{
        width: size,
        height: size,
        backgroundImage: "url('/sprites/draky-sheet.png')",
        backgroundSize: `${COLS * 100}% ${ROWS * 100}%`,
        backgroundPosition: `${x}% ${y}%`,
        backgroundRepeat: "no-repeat",
      }}
    />
  );
}
