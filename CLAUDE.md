# ProjectVTT

A modern, web-first virtual tabletop built as a from-scratch alternative to FoundryVTT. This is a for-fun project with no deadline — the priority is doing things well and learning, not shipping fast.

## Stack

- **Backend**: Rust, Axum, Tokio
- **Database**: SQLite via sqlx, one `.db` file per campaign world
- **Frontend**: TypeScript, Svelte 5 (runes syntax — see conventions below), Vite
- **Canvas**: Three.js with WebGPU renderer, WebGL2 fallback
- **Real-time sync**: WebSocket, single broadcast channel per server instance

## Architecture

The system is split into three layers, in order of dependency:

1. **Core** — game-system-agnostic. The server, database schema, WebSocket protocol, canvas rendering, and base UI. Core has zero knowledge of D&D, dice, hit points, or any specific ruleset.
2. **Plugin system** — the registration API that lets game systems hook into core. Defines TypeScript interfaces for actor schemas, item schemas, character sheet components, and dice resolvers.
3. **Plugins** — game system implementations. `dnd5e` is the first and primary target. Plugins implement the interfaces defined by the plugin system; core never imports anything from a plugin directly.

### Why this split matters

Foundry's biggest architectural weakness is that game systems can reach into core internals via inheritance and undocumented behavior, so core updates break systems unpredictably. We are avoiding that by enforcing the core/plugin boundary through TypeScript interfaces. A plugin can only interact with core through the defined API surface — never by reaching into core internals.

### Browser-side split

- **Canvas** (`client/src/canvas/`) — Three.js/WebGPU. Map, tokens, grid, lighting, fog. Pure visual rendering, no game logic.
- **UI** (`client/src/ui/`) — Svelte components rendered as normal DOM on top of the canvas. Character sheets, chat, sidebar, dice tray.
- **State** (`client/src/state/`) — shared reactive state (Svelte 5 runes in `.svelte.ts` files) that both canvas and UI read from and write to. Canvas and UI never communicate directly with each other.

### Server-side data flow

- **WebSocket** — real-time game events (token moves, dice rolls, fog updates, chat). Broadcast to all connected clients, persisted to SQLite alongside the broadcast.
- **REST** — load-once / save-on-demand operations (opening a character sheet, uploading a map, creating an actor). Not broadcast in real time.

### Database

- One SQLite file per campaign world (`world.db`).
- Schema is intentionally system-agnostic: `actors` and `items` tables store game-system-specific fields as a `data TEXT` JSON blob that core never inspects or validates. Only the relevant plugin interprets that blob.
- Core tables: `actors`, `items`, `scenes`, `messages`, `documents`, `world_meta`.
- Migrations live in `server/migrations/`, sequentially numbered. **Never edit a migration file once it has run against a database you care about** — write a new numbered migration instead. sqlx tracks applied migrations via checksum and will panic with `VersionMismatch` if a previously-applied file's contents change.
- Compendium-style reference data (spell lists, monster stat blocks) is planned to live in a separate plugin-owned `compendium.db`, distinct from world data. Not yet implemented.

## Deployment model

Single GM running the server, remote players connecting over the internet via port forwarding (a DigitalOcean droplet exists for future use but is not the current target). No auth system for v1 beyond a GM password concept (not yet implemented). The server compiles to a single self-contained binary — no Node.js or external runtime required at deploy time.

## Conventions

### Rust

- New-style module declarations: a module with submodules gets a `name.rs` file alongside a `name/` folder (not `name/mod.rs`).
- `AppState` lives in `state.rs`, wrapped in `Arc` and shared via Axum's `State` extractor.
- Shared data types (e.g. `GameEvent`) live in `models.rs`.
- Handlers are grouped by concern under `handlers/`, one file per route group.
- `Cargo.lock` IS committed (this is a binary project, not a library).

### Frontend

- **Svelte 5 runes only** — no Svelte 4 patterns. Use `$state`, `$derived`, `$effect`, `$props`. Do not use `export let` for props or implicit reactive `$:` statements.
- Use `onclick`, not `on:click` — Svelte 5 standard event attributes, not the deprecated directive syntax.
- Files needing runes outside a `.svelte` component use the `.svelte.ts` extension (e.g. `state/connection.svelte.ts`).
- When exporting `$state` from a module, wrap it in getter functions inside a returned object — destructuring `$state` directly loses reactivity outside the declaring module.
- `onMount` is still correct for one-time side effects on component mount (e.g. opening the WebSocket connection); it is not deprecated.

### Plugins

- A plugin is a TypeScript package under `plugins/<name>/`.
- Plugin schemas (e.g. `CharacterSchema`, `WeaponSchema`) are plain TypeScript interfaces describing data shape only — no methods, no logic.
- A plugin registers itself with core via a `defineSystem()` call (registration API not yet implemented as of this writing) providing: actor schemas + sheet components, item schemas + sheet components, a dice resolver, and combat/initiative handlers where relevant.
- The `dnd5e` plugin is being developed in parallel by a collaborator focusing on data model design (schemas), independent of the plugin-loading mechanism, which doesn't exist yet. This work is not wasted regardless of how the final loader is implemented.

## Requirements

Full requirements live in `docs/REQUIREMENTS.md`, organized by domain (Server, Database, WebSocket Protocol, Canvas, UI, Plugin System, dnd5e). Requirement IDs (e.g. `S-01`, `W-03`, `5E-06`) are referenced directly in code comments, commits, and GitHub issues — check there before assuming a feature is in or out of scope. An explicit "out of scope for v1" section at the bottom of that doc exists to prevent scope creep; check it before adding speculative features.

## v1 scope

Priority order: map + token movement, dice rolling (simple notation, no automation), read-only character sheets, fog of war, chat. Explicitly deferred: editable sheets, dice automation, initiative tracker, audio, hosting/auth hardening, plugin marketplace, mobile support.

## Current state

- Rust server compiles and runs as a single binary.
- SQLite connects with automatic migrations on startup.
- WebSocket endpoint at `/ws` with a single broadcast channel — all connected clients receive all messages (no session/game separation yet, by design, since v1 targets one GM running one world at a time).
- Health check at `/health`.
- Frontend has a minimal Svelte 5 test harness proving the WebSocket round-trip (`state/connection.svelte.ts`, `ui/Chat.svelte`).
- Three.js/WebGPU canvas not yet started — `canvas/Canvas.svelte` is currently an empty placeholder div.
- Plugin registration API (`defineSystem`) not yet implemented — collaborator is drafting `dnd5e` schemas in isolation ahead of this.

## Things to watch for

- If `cargo run` panics with `VersionMismatch`, a migration file was edited after running once. For local dev, deleting `world.db` and re-running is the fast fix; never do this against real campaign data.
- The WebSocket client does not currently auto-reconnect on disconnect — a dropped connection requires a manual page refresh. This is a known gap, not yet prioritized.
- Always start the Rust server before opening/refreshing browser tabs that connect via WebSocket, or you'll see `ERR_CONNECTION_REFUSED`.