import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";

type DrakyState = "idle" | "hover" | "processing" | "success" | "error";

const COLS = 8;
const ROWS = 8;

function pingPong(frames: [number, number][]): [number, number][] {
    if (frames.length <= 2) return frames;
    return [...frames, ...frames.slice(1, -1).reverse()];
}

const RAW_FRAMES: Record<DrakyState, [number, number][]> = {
    idle: [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
    hover: [[1, 4], [1, 5], [1, 6], [1, 7]],
    processing: [[2, 4], [2, 5], [3, 0], [3, 1], [3, 2], [1, 4], [1, 5]],
    success: [[3, 5], [3, 6], [7, 3], [7, 4]],
    error: [[1, 6], [1, 7], [7, 0]],
};

const SPRITE_MAP: Record<DrakyState, [number, number][]> = {
    idle: pingPong(RAW_FRAMES.idle),
    hover: pingPong(RAW_FRAMES.hover),
    processing: pingPong(RAW_FRAMES.processing),
    success: pingPong(RAW_FRAMES.success),
    error: pingPong(RAW_FRAMES.error),
};

function getFrameDelay(state: DrakyState, index: number, total: number): number {
    const isLast = index === total - 1;
    switch (state) {
        case "idle": return 120;
        case "hover": return 90;
        case "processing": return 80;
        case "success": return isLast ? 400 : 70;
        case "error": return isLast ? 400 : 70;
    }
}

export default function DrakyHero() {
    const [state, setState] = useState<DrakyState>("idle");
    const [spriteIndex, setSpriteIndex] = useState(0);

    // Auto-play routine to show how cute it is
    useEffect(() => {
        const routine = setInterval(() => {
            // Randomly change state to show off animations
            const possibleStates: DrakyState[] = ["idle", "idle", "hover", "success"];
            const randState = possibleStates[Math.floor(Math.random() * possibleStates.length)];
            setState(randState);
        }, 4500);
        return () => clearInterval(routine);
    }, []);

    useEffect(() => {
        const frames = SPRITE_MAP[state];
        setSpriteIndex(0);

        if (frames.length <= 1) return;

        let cancelled = false;
        let timeoutId: ReturnType<typeof setTimeout>;

        function tick(idx: number) {
            if (cancelled) return;
            const next = (idx + 1) % frames.length;
            const delay = getFrameDelay(state, idx, frames.length);
            timeoutId = setTimeout(() => {
                setSpriteIndex(next);
                tick(next);
            }, delay);
        }
        tick(0);

        return () => {
            cancelled = true;
            clearTimeout(timeoutId);
        };
    }, [state]);

    const [row, col] = SPRITE_MAP[state][spriteIndex] ?? SPRITE_MAP[state][0];
    const bgPosX = (col / (COLS - 1)) * 100;
    const bgPosY = (row / (ROWS - 1)) * 100;

    return (
        <div className="relative flex min-h-[400px] w-full items-center justify-center p-8">
            {/* Decorative background circle */}
            <motion.div
                className="absolute h-72 w-72 rounded-full bg-white opacity-60 mix-blend-multiply blur-3xl shadow-[0_0_100px_40px_rgba(45,212,191,0.3)]"
                animate={{
                    scale: [1, 1.1, 1],
                    opacity: [0.6, 0.8, 0.6],
                }}
                transition={{ duration: 4, repeat: Infinity, ease: "easeInOut" }}
            />

            {/* Sprite Container */}
            <motion.div
                className="relative z-10 flex cursor-pointer select-none items-center justify-center filter drop-shadow-[0_20px_20px_rgba(20,184,166,0.3)] hover:drop-shadow-[0_25px_25px_rgba(20,184,166,0.5)] transition-all duration-300"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                onHoverStart={() => setState("hover")}
                onHoverEnd={() => setState("idle")}
                onClick={() => {
                    setState("success");
                }}
            >
                <div
                    style={{
                        width: 256,
                        height: 256,
                        backgroundImage: `url(${import.meta.env.BASE_URL}sprites/draky-sheet.png)`,
                        backgroundSize: `${COLS * 100}% ${ROWS * 100}%`,
                        backgroundPosition: `${bgPosX}% ${bgPosY}%`,
                        backgroundRepeat: "no-repeat",
                    }}
                />

                {/* Playful Floating Badge */}
                <motion.div
                    className="absolute -right-4 top-4 rotate-12 rounded-full bg-pink-400 px-3 py-1 text-sm font-bold text-white shadow-lg shadow-pink-200"
                    animate={{ y: [-5, 5, -5] }}
                    transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
                >
                    Drag me!
                </motion.div>
            </motion.div>
        </div>
    );
}
