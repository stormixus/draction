You are the frontend/UI designer for **Draction** (drop + action), a desktop overlay app.

## Project Summary
Draction is a desktop app with a cute character overlay on the desktop. Users drag-and-drop files onto the character, which "swallows" them with an animation. Files are then automatically processed by rules and workflows. A companion panel shows run logs, status, and allows rule management.

## Key UI Requirements (from SPEC.md v0.1)
- **Overlay Character**: 128x128px default, user-draggable position, always-on-top, transparent background
- **Drop Interaction**:
  - Drag-over feedback: visual highlight (enlarge/glow)
  - "Swallowing" animation on drop
  - Progress indicator for large files (>100MB)
  - Status display: eating/success/fail states
- **Toast Notifications**: Brief status + "Details" click to open log panel
- **Log Panel**: Run history (success/fail/time), node-level details
- **Settings Panel**: Dangerous features toggle, path scopes, token management

## Your Task
Provide a **concrete UI/UX design specification**:

1. **Overlay Character Design Concept**:
   - Visual style recommendation (pixel art? vector? lottie animation?)
   - State animations: idle, hover, eating, success, fail
   - Size and positioning behavior

2. **Component Architecture** (for Tauri/Electron + React/Svelte):
   - Component tree with responsibilities
   - Recommended UI framework (React vs Svelte vs Solid)
   - Styling approach (Tailwind? CSS Modules? styled-components?)

3. **Animation System**:
   - How to implement the "swallowing" animation
   - Drag-over glow/enlarge effect
   - Progress ring for large files
   - Success/fail feedback animations
   - Recommended animation libraries

4. **Window Architecture**:
   - Overlay window (transparent, click-through except on character)
   - Main panel window (log, settings, rule management)
   - How the two windows interact

5. **Design Tokens / Theme**:
   - Color palette suggestion (fun but professional)
   - Typography
   - Spacing and sizing system

Respond in Korean. Be specific — provide component names, CSS approaches, animation keyframe concepts. This will be used to start building immediately.
