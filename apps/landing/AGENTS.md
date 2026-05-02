<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# landing

## Purpose
Astro 5 + React 19 marketing/landing site for Draction. Static-rendered pages with embedded interactive React components (`DrakyHero`, `ThemeToggle`). Tailwind 4 for styling. Currently a single `index.astro` page.

## Key Files
| File | Description |
|------|-------------|
| `package.json` | pnpm workspace `apps-landing`; scripts: `dev` (astro dev), `build` (astro build), `preview` |
| `astro.config.mjs` | Astro config — registers `@astrojs/react` and Tailwind |
| `tsconfig.json` | TS config for the Astro/React mix |
| `README.md` | Stock Astro starter README |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | Pages, layouts, components, global styles (see `src/AGENTS.md`) |
| `public/` | Static assets — favicons + Draky sprite sheet served at site root |
| `.astro/` | Astro-generated content collection types — gitignored |
| `.vscode/` | Editor settings |

## For AI Agents

### Working In This Directory
- Pages are `.astro` files; interactive components are `.tsx` and must be opted into hydration with `client:load` / `client:idle` directives in their parent `.astro`.
- The repo root `pnpm build` does **not** build this app — it only builds `draction-desktop`. Build this site explicitly with `pnpm -F apps-landing build`.
- Sprite assets here mirror `apps/draction-desktop/public/sprites/draky-sheet.png` — when regenerating sprites via `scripts/`, update both copies.

### Testing Requirements
- `pnpm -F apps-landing dev` for local preview at `http://localhost:4321`.
- No automated tests; visual review is the verification path.

### Common Patterns
- Astro for shell + SSG; React only where interactivity is needed.
- Tailwind 4 via `@tailwindcss/vite` (no `tailwind.config.js`).
- `lucide-react` for icons.

## Dependencies

### External
- Astro 5.18, `@astrojs/react` 4.4, React 19.2, Tailwind 4.2, framer-motion 11.18, lucide-react 0.577

<!-- MANUAL: -->
