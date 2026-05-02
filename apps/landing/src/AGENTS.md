<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src

## Purpose
Landing-site source: a single Astro page, a shared layout, two interactive React components, and global CSS.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `components/` | React components: `DrakyHero.tsx` (animated hero), `ThemeToggle.tsx` (light/dark toggle) |
| `layouts/` | `Layout.astro` — base HTML shell wrapping every page |
| `pages/` | `index.astro` — the only page at `/` |
| `styles/` | `global.css` — Tailwind imports + custom variables |

## For AI Agents

### Working In This Directory
- `.astro` files render statically by default. React components only hydrate when explicitly opted in with `client:load`/`client:idle`/`client:visible` in their parent `.astro`.
- New routes are new files under `pages/` (e.g. `pages/about.astro` becomes `/about`).
- Tailwind is configured via `@tailwindcss/vite`; styles are scoped through Astro's component-scoped CSS or written into `styles/global.css`.

### Testing Requirements
- `pnpm -F apps-landing dev` for live preview at `localhost:4321`.

### Common Patterns
- Astro for layout/SSG; React for animation-heavy interactive widgets.
- Frontmatter in `.astro` files (the `---` block at top) is server-side TS; the body is the rendered HTML/JSX.

## Dependencies

### External
- Astro 5, React 19, Tailwind 4, framer-motion, lucide-react

<!-- MANUAL: -->
