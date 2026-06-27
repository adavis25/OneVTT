# OneVTT

A modern, web-first virtual tabletop built as a from-scratch alternative to FoundryVTT. This is a for-fun project with no deadline — the priority is doing things well and learning, not shipping fast.

---

## Stack

- **Backend**: Rust, Axum, Tokio
- **Database**: SQLite via sqlx, one `.db` file per campaign world
- **Frontend**: TypeScript, Svelte 5 (runes syntax — see conventions below), SvelteKit (static adapter, SPA mode), Vite
- **Canvas**: Three.js with WebGL renderer (WebGPU renderer planned, WebGL current), orthographic camera for top-down view
- **Real-time sync**: WebSocket, single broadcast channel per server instance
- **Styling**: Tailwind CSS v4 via `@tailwindcss/vite`

---

## Architecture

The system is split into three layers, in order of dependency:

1. **Core** — game-system-agnostic. The server, database schema, WebSocket protocol, canvas rendering, and base UI. Core has zero knowledge of D&D, dice, hit points, or any specific ruleset.
2. **Plugin system** — the registration API that lets game systems hook into core. Defines TypeScript interfaces for actor schemas, item schemas, character sheet components, and dice resolvers.
3. **Plugins** — game system implementations. `dnd5e` is the first and primary target. Plugins implement the interfaces defined by the plugin system; core never imports anything from a plugin directly.

### Why this split matters

Foundry's biggest architectural weakness is that game systems can reach into core internals via inheritance and undocumented behavior, so core updates break systems unpredictably. We enforce the core/plugin boundary through TypeScript interfaces — a plugin can only interact with core through the defined API surface.

### Browser-side split

- **Canvas** (`lib/components/canvas/`) — Three.js. Map, tokens, grid, lighting, fog. Pure visual rendering, no game logic. Fills the full viewport behind the UI layer.
- **UI** (`lib/components/ui/`) — Svelte components rendered as normal DOM absolutely positioned on top of the canvas. Chat, sidebars, dice tray, character sheets.
- **Primitives** (`lib/components/primitives/`) — reusable UI building blocks (Button, Panel, Input etc.). Used by UI components.
- **State** (`lib/state/`) — shared reactive state (Svelte 5 runes in `.svelte.ts` files) that both canvas and UI read from and write to. Canvas and UI never communicate directly.

### Routing

SvelteKit file-based routing with `adapter-static` in SPA mode. `fallback: 'index.html'` handles client-side navigation. Routes:

```
/           → Welcome screen (routes/+page.svelte + routes/Welcome.svelte)
/game       → Canvas + UI game view (routes/game/+page.svelte + routes/game/Game.svelte)
/settings   → Settings + component showcase (routes/settings/+page.svelte + routes/settings/Settings.svelte)
```

Page-level components are co-located with their route files, not in `lib/`. Only shared components live in `lib/components/`.

### Server-side data flow

- **WebSocket** — real-time game events (token moves, dice rolls, fog updates, chat messages). Broadcast to all connected clients, persisted to SQLite alongside the broadcast.
- **REST** — load-once / save-on-demand operations (opening a character sheet, uploading a map, creating an actor). Not broadcast in real time.

### Database

- One SQLite file per campaign world (`world.db`).
- Schema is intentionally system-agnostic: `actors` and `items` tables store game-system-specific fields as a `data TEXT` JSON blob that core never inspects or validates. Only the relevant plugin interprets that blob.
- Core tables: `actors`, `items`, `scenes`, `messages`, `documents`, `world_meta`.
- Migrations live in `server/migrations/`, sequentially numbered. **Never edit a migration file once it has run against a database you care about** — write a new numbered migration instead. sqlx tracks applied migrations via checksum and will panic with `VersionMismatch` if a previously-applied file's contents change.
- If `VersionMismatch` occurs during dev, deleting `world.db` and restarting is the fast fix.
- Compendium reference data (spell lists, monster stat blocks) will live in separate plugin-owned `.db` files (e.g. `plugins/dnd5e/compendium/spells.db`), opened read-only alongside `world.db`. Not yet implemented.

---

## Project structure

```
OneVTT/
├── server/                          ← Rust backend
│   ├── migrations/
│   │   └── 0001_initial_schema.sql
│   ├── src/
│   │   ├── main.rs                  ← entry point, wires everything
│   │   ├── state.rs                 ← AppState (db pool + broadcast sender)
│   │   ├── models.rs                ← shared data types (GameEvent etc.)
│   │   ├── handlers.rs              ← declares handler submodules
│   │   └── handlers/
│   │       ├── health.rs            ← GET /health
│   │       └── ws.rs                ← WebSocket handler + broadcast logic
│   └── Cargo.toml
│
├── client/                          ← SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── canvas/
│   │   │   │   │   └── Canvas.svelte       ← Three.js scene, grid, tokens
│   │   │   │   ├── ui/                     ← feature UI components
│   │   │   │   │   ├── Chat.svelte
│   │   │   │   │   ├── SidebarLeft.svelte
│   │   │   │   │   └── SidebarRight.svelte
│   │   │   │   └── primitives/             ← reusable building blocks
│   │   │   ├── state/
│   │   │   │   ├── connection.svelte.ts    ← WebSocket connection + message state
│   │   │   │   └── theme.svelte.ts         ← light/dark theme state + localStorage
│   │   │   ├── index.ts
│   │   │   └── utils.ts                    ← cn() Tailwind class merger
│   │   ├── routes/
│   │   │   ├── layout.css                  ← Tailwind import, @theme tokens, :root/:dark vars
│   │   │   ├── Welcome.svelte              ← welcome screen component
│   │   │   ├── +page.svelte                ← / route
│   │   │   ├── +layout.svelte              ← root layout (initialises theme on mount)
│   │   │   ├── game/
│   │   │   │   ├── Game.svelte             ← game layout component
│   │   │   │   └── +page.svelte            ← /game route, starts WebSocket
│   │   │   └── settings/
│   │   │       ├── Settings.svelte         ← theme toggle + primitive showcase
│   │   │       └── +page.svelte            ← /settings route
│   │   └── app.html                        ← class="dark" default, no inline script needed
│   ├── svelte.config.js                    ← adapter-static, SPA mode
│   └── vite.config.ts                      ← Tailwind plugin, WS/API proxy
│
├── plugins/
│   └── dnd5e/
│       └── src/
│           └── schemas/                    ← TypeScript interfaces (in progress)
│
├── docs/
├── CLAUDE.md
└── README.md
```

---

## Deployment model

Single GM running the server, remote players connecting over the internet via port forwarding. No auth system for v1. The server compiles to a single self-contained binary — no Node.js or external runtime required at deploy time. DigitalOcean droplet exists for future hosted deployment.

---

## Conventions

### Rust

- New-style module declarations: a module with submodules gets a `name.rs` file alongside a `name/` folder (not `name/mod.rs`).
- `AppState` lives in `state.rs`, wrapped in `Arc` and shared via Axum's `State` extractor.
- Shared data types (e.g. `GameEvent`) live in `models.rs`.
- Handlers are grouped by concern under `handlers/`, one file per route group.
- `Cargo.lock` IS committed (this is a binary project, not a library).
- Static files served from `../client/.svelte-kit/output/client/` in production via `ServeDir` with `index.html` fallback.

### Frontend

- **Svelte 5 runes only** — no Svelte 4 patterns. Use `$state`, `$derived`, `$effect`, `$props`. Do not use `export let` for props or implicit reactive `$:` statements.
- Use `onclick`, not `on:click` — Svelte 5 standard event attributes.
- Files needing runes outside a `.svelte` component use the `.svelte.ts` extension.
- When exporting `$state` from a module, wrap in getter functions inside a returned object — destructuring loses reactivity outside the declaring module.
- `onMount` is correct for one-time side effects on component mount (e.g. starting the WebSocket connection, initializing Three.js).
- Page-level components co-locate with their route files in `src/routes/`, not in `lib/`.
- Shared components live in `src/lib/components/` only if used by more than one route.
- Tailwind v4 — no `tailwind.config.js`. All config via `@theme` directives in `routes/layout.css`.

### Theme system

Theming uses CSS custom properties as design tokens, mapped into Tailwind utilities via `@theme` in `routes/layout.css`. Never use hardcoded Tailwind color classes (e.g. `bg-gray-900`) in UI code — always use semantic token classes so pages respond to theme changes.

**Available token classes** (all follow `bg-*` / `text-*` / `border-*` / `ring-*` patterns):

| Token | Purpose |
|---|---|
| `background` / `foreground` | Page background and primary text |
| `card` / `card-foreground` | Elevated surfaces (panels, dialogs) |
| `primary` / `primary-foreground` | Brand accent, primary actions |
| `secondary` / `secondary-foreground` | Subdued interactive areas |
| `muted` / `muted-foreground` | Placeholder text, disabled states, subtitles |
| `accent` / `accent-foreground` | Hover states on ghost/outline elements |
| `destructive` / `destructive-foreground` | Danger actions |
| `border` | Dividers, input borders |
| `input` | Input field border specifically |
| `ring` | Focus ring color |

**`--radius`** — border radius design token. Use `rounded-[var(--radius)]` in components rather than a fixed class so it can be changed globally.

**Theme state** (`lib/state/theme.svelte.ts`) — call `getTheme()` to read `theme.current` (`'light' | 'dark'`) or call `theme.set(t)` / `theme.toggle()`. Persists to `localStorage` under key `pvtt-theme`. Defaults to `'dark'`. Initialisation (`theme.init()`) runs once in `+layout.svelte` on every page.

**Making a new page theme-aware:** use `bg-background text-foreground` on the outermost wrapper. Use token classes for all colors — no hardcoded Tailwind palette classes.

**Canvas background:** `Canvas.svelte` uses `alpha: true` on the WebGL renderer (no `scene.background`) so the canvas is transparent and the `bg-background` CSS class on its container div shows through. Do not set `scene.background` — let CSS control it.

### Primitives

Primitives live in `lib/components/primitives/`. Existing: `Button`, `Input`, `Label`. Follow this pattern for new ones:

- Accept all native HTML element attributes by extending the relevant `HTML*Attributes` type from `svelte/elements`.
- Destructure `class: className` and spread `...rest` onto the root element so callers can extend styles.
- Use `cn()` (from `$lib/utils`) to merge classes. Put base styles first, variant/size maps second, `className` last.
- If the element has a `value` that callers may want to bind, declare it as `value = $bindable()` in `$props()` and use `bind:value` on the native element.
- Do not hardcode colors — use semantic token classes only.
- Do not add wrapper divs unless the component genuinely needs them.

Example skeleton:
```svelte
<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends HTMLButtonAttributes { children?: Snippet }
  let { class: className, children, ...rest }: Props = $props();
</script>

<button class={cn('/* base styles */', className)} {...rest}>
  {@render children?.()}
</button>
```

### Component organization

- `lib/components/canvas/` — Three.js canvas components
- `lib/components/ui/` — feature UI components (Chat, Sidebar, DiceTray)
- `lib/components/primitives/` — reusable building blocks (Button, Input, Panel)
- `routes/<route>/` — page-level components co-located with their route

### Plugins

- A plugin is a TypeScript package under `plugins/<name>/`.
- Plugin schemas are plain TypeScript interfaces — data shape only, no methods or logic.
- A plugin registers itself with core via `defineSystem()` (not yet implemented).
- `defineSystem()` is called after world load, once the client knows which game system the world uses. It locks after the first call — only one system can be active per session.
- The `dnd5e` plugin schemas are being developed in parallel, independent of the plugin loader.

---

## Licensing

- **Core application** — Proprietary (commercial)
- **Game system plugins** — GPL v3
- **SRD content** — CC BY 4.0 (SRD 5.1 and 5.2). Only SRD content ships in the default compendium. Non-SRD content is user-provided.

---

## Current state

- Rust server compiles and runs as a single binary on port 3000.
- SQLite connects with automatic migrations on startup. `world.db` created on first run.
- WebSocket endpoint at `/ws` — single broadcast channel, all connected clients receive all events. Connect/disconnect logged via `tracing`.
- Health check at `/health` returns `ok`.
- SvelteKit frontend running on port 5173 in dev (proxies `/api` and `/ws` to Rust server).
- Welcome screen at `/` with GM/Player role toggle and world name input. Navigates to `/game` on submit. Uses Button, Input, Label primitives and theme tokens.
- Three.js canvas rendering at `/game` — orthographic top-down view, 200-unit grid, `ResizeObserver` handles resize. Canvas is alpha-transparent; background comes from CSS `bg-background`. Left-drag to pan, scroll wheel to zoom toward cursor.
- WebSocket connects on `/game` route mount via `onMount` in `+page.svelte`.
- Theme system live — light/dark toggle in `/settings`, persisted to `localStorage`, initialised globally in `+layout.svelte`. All pages and primitives use semantic token classes.
- Primitives implemented: `Button` (5 variants, 4 sizes), `Input` (bindable value), `Label`.
- `/settings` page is the component showcase and theme control surface.
- `lib/state/connection.svelte.ts` manages WebSocket connection and message state.
- Custom sidebar not yet started (shadcn-svelte sidebar was evaluated and removed).
- Plugin registration API (`defineSystem`) not yet implemented.
- dnd5e schemas being drafted by collaborator.

---

## Things to watch for

- If `cargo run` panics with `VersionMismatch`, delete `world.db` and restart. Never do this against real campaign data.
- Always start the Rust server before opening browser tabs — WebSocket connection fails with `ERR_CONNECTION_REFUSED` otherwise.
- WebSocket client does not auto-reconnect on disconnect — known gap, not yet prioritized.
- The `svelte.config.js` should not import `vitePreprocess` — SvelteKit handles preprocessing internally. Removing it fixed a recurring native binding error.
- The npm optional dependency bug (`Cannot find native binding`) is resolved by `rm -rf node_modules package-lock.json && npm install`. If it recurs, check for packages with broken optional deps.
- `package-lock.json` IS committed (app project, not a library).