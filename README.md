<div align="center">
  <img src="docs/assets/hero.png" alt="Draction Banner">
</div>

# Draction 🐾

> A tiny, magical fairy living on your desktop! It happily munches on your files and magically organizes them for you! 🪄✨

Draction is your super smart, ridiculously cute desktop companion that loves to play on your screen! 
Just toss your files into this little friend hovering on your desktop, and it will automatically sort them into folders, rename them, or even convert videos for you. It's an adorably diligent little helper that works perfectly all on its own! ฅ^•ﻌ•^ฅ

*(Note: In the future, it might even team up with a smart AI kitty friend named OpenClaw! But for now, it's doing a fantastic job solo!)*

---

## 🌟 MVP Features & Cuteness Overload

- **Kawaii Desktop Portal (Drop & Ingest)**: A soft, glowing, bouncy portal that floats on your screen with a smile. Drag and drop your files, and it will gulp them down into its `~/Draction/Inbox` tummy! Yum!
- **Smart Little Brain (Rule Engine)**: It looks at the conditions (like file extensions or size) and decides the best way to handle your files. It knows exactly what chore to do!
- **Busy Bee Workflows (n8n-lite)**: A tiny, hard-working workflow engine! Even completely offline, it can efficiently move, copy, rename, or transcode your files without breaking a sweat.
- **Oopsie Daisy! (Undo Feature)**: Dropped the wrong file? Don't panic! You have 10 seconds to say "Oops!" and Draction will gently spit it back out right where it was.

---

## 🏛 Fluffy Architecture

<div align="center">
  <img src="docs/assets/architecture.png" alt="Draction Architecture (Super Cute Version)">
  <br><br>
  <i>Draction friends working hard in the clouds ☁️💖</i>
</div>

### 1. Draction Desktop App 🎒
- **Overlay Window**: The soft, squishy, transparent drop-zone jelly on your desktop.
- **Inbox Manager**: Safely stores the files it ate, and gently returns them if you make an "Oopsie!" request.
- **Engine Core (Rule + Workflow)**: The brilliant little heart that says "This file goes there!" and executes chores in the correct order.

---

## ⚙ How the Magic Works (Ingest Pipeline)

1. **Plop! (Drop Event)**: You toss a file into the squishy overlay portal! 🍬
2. **Spreading the Word (Event Trigger)**: It joyfully announces "Yay! A file arrived!" internally (`EVENT_INGESTED`).
3. **Finding a Buddy (Rule Matching)**: It looks through its rulebook to find the perfect little worker for this specific file.
4. **Chores Done! (Run/Status)**: Once the workflow finishes its hard work, it leaves behind a pretty little log for you to check. 🥰

---

## 🛣 Adorable Roadmap
- [x] **v0.1 (MVP)**: Desktop jelly overlay + file munching + busy bee workflows (Standalone mode). (We are here!)
- [ ] **v0.2**: Handling multiple rules at once, smarter priorities, and an "I'll try again!" (Retry) feature when things go wrong.
- [ ] **v0.3**: Teaming up with a super smart AI bestie, OpenClaw! 🐾

> **Made with lots of Love and Nano Banana 2 ✨🍌💖**
