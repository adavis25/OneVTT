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
- Core tables: `actors`, `items`, `scenes`, `messages`, `documents`, `world_meta`, `worlds`.
- Migrations live in `server/migrations/`, sequentially numbered. **Never edit a migration file once it has run against a database you care about** — write a new numbered migration instead. sqlx tracks applied migrations via checksum and will panic with `VersionMismatch` if a previously-applied file's contents change.
- If `VersionMismatch` occurs during dev, deleting `world.db` and restarting is the fast fix.
- Compendium reference data (spell lists, monster stat blocks) will live in a single plugin-owned `dnd5e.db` file opened read-only alongside `world.db`. Not yet implemented.
- `.sqlx/` folder IS committed — contains cached query metadata for compile-time verification without needing a live DB.
- `server/.env` contains `DATABASE_URL=sqlite:world.db?mode=rwc` — not committed, but `server/.env.example` is.

---

## Project structure

```
OneVTT/
├── server/                          ← Rust backend
│   ├── migrations/
│   │   ├── 0001_initial_schema.sql  ← core tables
│   │   └── 0002_worlds.sql          ← worlds table
│   ├── src/
│   │   ├── main.rs                  ← entry point, wires everything
│   │   ├── state.rs                 ← AppState (db pool + broadcast sender)
│   │   ├── models.rs                ← shared data types (GameEvent etc.)
│   │   ├── handlers.rs              ← declares handler submodules
│   │   └── handlers/
│   │       ├── health.rs            ← GET /health
│   │       ├── ws.rs                ← WebSocket handler + broadcast logic
│   │       ├── world.rs             ← GET/POST/DELETE /api/worlds
│   │       └── actors.rs            ← GET/POST /api/actors
│   └── Cargo.toml
│
├── client/                          ← SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── canvas/
│   │   │   │   │   └── Canvas.svelte       ← Three.js scene, grid, pan/zoom
│   │   │   │   ├── ui/                     ← feature UI components
│   │   │   │   │   ├── Chat.svelte
│   │   │   │   │   ├── SidebarLeft.svelte  ← world name + game system
│   │   │   │   │   └── SidebarRight.svelte ← Actors/Items/Journal tabs
│   │   │   │   └── primitives/             ← Button, Input, Label
│   │   │   ├── state/
│   │   │   │   ├── connection.svelte.ts    ← WebSocket connection + message state
│   │   │   │   ├── world.svelte.ts         ← active world state (id, name, game_system)
│   │   │   │   └── theme.svelte.ts         ← light/dark theme + localStorage
│   │   │   ├── index.ts
│   │   │   └── utils.ts                    ← cn() Tailwind class merger
│   │   ├── routes/
│   │   │   ├── layout.css                  ← Tailwind import, @theme tokens, :root/:dark vars
│   │   │   ├── Welcome.svelte              ← world select/create/delete UI
│   │   │   ├── +page.svelte                ← / route
│   │   │   ├── +layout.svelte              ← root layout, initialises theme
│   │   │   ├── game/
│   │   │   │   ├── Game.svelte             ← canvas + left/right sidebars
│   │   │   │   └── +page.svelte            ← /game route, starts WebSocket, loads plugin
│   │   │   └── settings/
│   │   │       ├── Settings.svelte         ← theme toggle + primitive showcase
│   │   │       └── +page.svelte            ← /settings route
│   │   └── app.html
│   ├── svelte.config.js                    ← adapter-static, fallback: 'index.html'
│   └── vite.config.ts                      ← tailwindcss(), sveltekit(), proxy config
│
├── plugins/
│   └── dnd5e/
│       └── src/
│           ├── index.ts                    ← calls defineSystem() with stub
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
- `.sqlx/` IS committed — cached query metadata for compile-time verification.
- Write handlers incrementally — `cargo check` after each function. Switch to `query!` macros last, then run `cargo sqlx prepare`. Never write a full handler and check at the end.
- After adding new `query!` macros, always run `cargo sqlx prepare` to update the cache.

### Frontend

- **Svelte 5 runes only** — no Svelte 4 patterns. Use `$state`, `$derived`, `$effect`, `$props`. Do not use `export let` for props or implicit reactive `$:` statements.
- Use `onclick`, not `on:click` — Svelte 5 standard event attributes.
- Files needing runes outside a `.svelte` component use the `.svelte.ts` extension.
- When exporting `$state` from a module, wrap in getter functions inside a returned object — destructuring loses reactivity outside the declaring module.
- `onMount` is correct for one-time side effects on component mount.
- Page-level components co-locate with their route files in `src/routes/`, not in `lib/`.
- Shared components live in `src/lib/components/` only if used by more than one route.
- Tailwind v4 — no `tailwind.config.js`. All config via `@theme` directives in `routes/layout.css`.

### Theme system

Theming uses CSS custom properties as design tokens, mapped into Tailwind utilities via `@theme` in `routes/layout.css`. Never use hardcoded Tailwind color classes (e.g. `bg-gray-900`) in UI code — always use semantic token classes.

**Available token classes:**

| Token | Purpose |
|---|---|
| `background` / `foreground` | Page background and primary text |
| `card` / `card-foreground` | Elevated surfaces (panels, dialogs) |
| `primary` / `primary-foreground` | Brand accent, primary actions |
| `secondary` / `secondary-foreground` | Subdued interactive areas |
| `muted` / `muted-foreground` | Placeholder text, disabled states |
| `accent` / `accent-foreground` | Hover states on ghost/outline elements |
| `destructive` / `destructive-foreground` | Danger actions |
| `border` | Dividers, input borders |
| `input` | Input field border |
| `ring` | Focus ring color |

**`--radius`** — use `rounded-[var(--radius)]` not a fixed class.

**Theme state** (`lib/state/theme.svelte.ts`) — `getTheme()` returns `{ current, set(), toggle(), init() }`. Persists to `localStorage` under `pvtt-theme`. Defaults to `dark`. `init()` runs in `+layout.svelte`.

**Canvas background** — `Canvas.svelte` uses `alpha: true` on the WebGL renderer. Do not set `scene.background` — let CSS `bg-background` on the container control it.

### Primitives

Primitives live in `lib/components/primitives/`. Existing: `Button` (5 variants, 4 sizes), `Input` (bindable), `Label`.

Pattern for new primitives:
- Extend relevant `HTML*Attributes` from `svelte/elements`
- Destructure `class: className`, spread `...rest` onto root element
- Use `cn()` to merge classes — base styles first, variants second, `className` last
- Bindable values use `$bindable()` in `$props()`
- Semantic token classes only, no hardcoded palette colors
- No unnecessary wrapper divs

### Plugin system

- Plugin registry lives in `lib/state/registry.svelte.ts` — exports `defineSystem()` and `getSystem()`
- `defineSystem()` locks after first call — second call logs warning and returns
- Called from `/game` route after world load, based on `activeWorld.game_system`
- `plugins/dnd5e/src/index.ts` calls `defineSystem()` with stub GameSystem object
- Full `GameSystem` interface includes: `id`, `label`, `ui` (actorSheets, actorCreators, itemSheets, sidebarPanels, diceTray), `dice`

### Component organization

- `lib/components/canvas/` — Three.js canvas components
- `lib/components/ui/` — feature UI components (Chat, SidebarLeft, SidebarRight)
- `lib/components/primitives/` — reusable building blocks
- `routes/<route>/` — page-level components co-located with route

---

## Licensing

- **Core application** — Proprietary (commercial)
- **Game system plugins** — GPL v3
- **SRD content** — CC BY 4.0 (SRD 5.1 and 5.2). Only SRD content in default compendium.

---

## Current state

- Rust server compiles and runs as single binary on port 3000
- SQLite auto-migrates on startup, `world.db` created on first run
- WebSocket at `/ws` — broadcast channel, connect/disconnect logged
- Health check at `/health`
- **Worlds API working** — `GET /api/worlds`, `POST /api/worlds`, `DELETE /api/worlds/:id`
- **Actors API working** — `GET /api/actors?world_id=:id`, `POST /api/actors`
- WebSocket broadcasts `actor.created` events after POST
- Welcome screen — world select/create/delete, navigates to `/game` with active world set in state
- Active world persists on page refresh via state
- Three.js canvas — orthographic top-down grid, pan with left-drag, zoom to cursor with scroll wheel, alpha-transparent background
- Left sidebar — shows current world name and game system
- Right sidebar — Actors/Items/Journal tabs, actors list populated from API, real-time updates via WebSocket
- Theme system — light/dark toggle, persisted to localStorage, all pages use semantic tokens
- Primitives: Button, Input, Label
- Plugin registry (`registry.svelte.ts`) — `defineSystem()` and `getSystem()` implemented, locks after first call
- dnd5e plugin stub (`plugins/dnd5e/src/index.ts`) — calls `defineSystem()` with stub on world load
- `/settings` — theme toggle + component showcase

## What's next (priority order)

1. **`GET /api/actors/:id`** — fetch single actor by ID
2. **`PATCH /api/actors/:id`** — update actor data blob
3. **`DELETE /api/actors/:id`** — remove actor
4. **Actor sheet component** — clicking actor in list opens read-only sheet, mounted by core via plugin registry
5. **Character creator** — dnd5e plugin provides `CharacterCreator.svelte`, registered via `actorCreators` slot
6. **Compendium endpoints** — `GET /api/compendium/races`, `GET /api/compendium/classes` etc. for character creation
7. **Token system** — place actors as tokens on canvas, sync movement via WebSocket
8. **dnd5e schemas** — TypeScript interfaces for character, NPC, weapon, spell (collaborator working on this)

---

## Common gotchas

| Problem | Fix |
|---|---|
| `VersionMismatch` panic | Delete `world.db`, restart. Never on real campaign data. |
| `ERR_CONNECTION_REFUSED` | Start Rust server before opening browser. |
| `Cannot find native binding` | `rm -rf node_modules package-lock.json && npm install` |
| `onclick` error in VS Code | Svelte: Restart Language Server (Ctrl+Shift+P) |
| `vitePreprocess` in svelte.config.js | Remove it — SvelteKit handles preprocessing internally |
| sqlx query macro error | Set `DATABASE_URL` in `server/.env`, run `cargo sqlx prepare` |
| New query macro not compiling | Run `cargo sqlx prepare` to update `.sqlx/` cache |
| `.svelte-kit/` in git | Add `client/.svelte-kit/` to `.gitignore` |

---

## Dev workflow

```bash
# Terminal 1 — Rust backend
cd server
cargo run

# Terminal 2 — SvelteKit frontend
cd client
npm run dev
```

Backend on port 3000, frontend on port 5173. Vite proxies `/api` and `/ws` to Rust server.