<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# components

## Purpose
Shared React components rendered inside the dashboard window's tabs.

## Key Files
| File | Description |
|------|-------------|
| `RunsPanel.tsx` | Lists runs from `GET /api/v1/runs` — status, workflow id, started/finished timestamps |
| `RulesPanel.tsx` | Lists rules from `GET /api/v1/rules` and renders enabled/disabled state |

## For AI Agents

### Working In This Directory
- Both panels accept a `baseUrl: string` prop (the discovered API base) and refetch periodically. They are deliberately **read-only at v0.1** — there is no inline rule editor.
- Errors from `fetch` are rendered inline rather than thrown — keep that pattern; the dashboard should never crash because the API is briefly unreachable.

### Testing Requirements
- No component tests at v0.1.

### Common Patterns
- Functional components with `useEffect`-driven polling.
- Tailwind utility classes — avoid inline `style` props.

## Dependencies

### Internal
- Calls into `crates/draction-api` over HTTP

### External
- React 19

<!-- MANUAL: -->
