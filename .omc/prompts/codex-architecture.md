You are the backend architect for **Draction** (drop + action), a desktop overlay app.

## Project Summary
Draction is a desktop app where users drop files onto an overlay character. Files are moved to an Inbox, matched against rules (Hazel-style condition tree), and processed through workflows (n8n-lite node graph). A companion app "OpenClaw" connects via local HTTP+WebSocket API for AI-powered rule creation.

## Key Requirements (from SPEC.md v0.1 MVP)
- Desktop overlay: transparent, always-on-top, drag-and-drop target
- Inbox: file move/copy with undo (10s, 5-item stack)
- Rule Engine: condition tree, first-match-wins, serial execution
- Workflow Engine: 5 nodes (move, copy, rename, transcode/ffmpeg, webhook or s3_upload), fail-fast
- Local API server: REST endpoints (rules CRUD, workflows CRUD, runs log, events) + WebSocket
- SQLite for event log + runs DB
- Process lifecycle: single instance (lock file), crash recovery, state.json
- Auth: Bearer token, localhost only

## Your Task
Provide a **concrete technical architecture recommendation**:

1. **Tech Stack Decision**: Choose between Tauri v2 + Rust/TypeScript vs Electron + TypeScript. Justify with specific reasons for this project's overlay + workflow engine requirements.

2. **Project Structure**: Provide exact directory tree for a monorepo setup.

3. **Backend Core Design** (the Rust or Node.js side):
   - Rule Engine: data structures, evaluation algorithm
   - Workflow Engine: executor pattern, node interface, error handling
   - API Server: framework choice, middleware stack
   - SQLite schema: tables for events, rules, workflows, runs

4. **Key Implementation Decisions**:
   - How to implement the transparent overlay with drag-and-drop
   - How to handle ffmpeg transcode (subprocess? bundled binary?)
   - WebSocket event broadcasting pattern
   - Lock file + crash recovery mechanism

Respond in Korean. Be specific — provide actual code structures, not just descriptions. This will be used to scaffold the project immediately.
