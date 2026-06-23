# Project structure

This document explains what each part of the repository is, what it does, and why it exists. It is written for someone unfamiliar with the tools and conventions used in this project.

---

## The repository

```
vtt/
├── server/
├── client/
├── plugins/
└── docs/
```

This is a **monorepo** — one git repository that holds multiple distinct projects. The alternative would be separate repos for the frontend and backend, but keeping them together means one git history, easier cross-referencing, and collaborators only need to clone one thing.

---

## server/

```
server/
├── migrations/
│   └── 0001_initial_schema.sql
├── src/
│   └── main.rs
└── Cargo.toml
```

This is the Rust project. It compiles into a single binary that is the entire backend of the application — the web server, the database connection, the WebSocket handler, and the static file server all live here.

### Cargo.toml

Rust's equivalent of `package.json`. Defines the project name, version, and dependencies. When you run `cargo build`, Cargo reads this file, downloads the listed packages (called **crates** in Rust) from [crates.io](https://crates.io), compiles them, and links them into your binary. You never manually download or manage Rust dependencies — Cargo handles everything.

### src/main.rs

The entry point of the Rust program. When you run the compiled binary, execution starts here. This file starts the database connection, sets up the web server, and begins listening for incoming connections. As the project grows, `main.rs` will stay lean — most logic moves into separate modules that `main.rs` wires together.

### migrations/

SQL files that define and evolve the database schema over time. Each file is numbered sequentially. When the server starts, sqlx checks which migration files have already been applied to the current database and runs only the new ones. This means you can add a new migration file later and existing world files will be upgraded automatically on next launch without losing any data.

---

## client/

```
client/
├── src/
│   ├── canvas/
│   ├── ui/
│   └── state/
├── package.json
└── vite.config.ts
```

This is the frontend project. It is written in TypeScript and Svelte and compiles into static HTML, CSS, and JavaScript files. The Rust server serves these files to browsers — the browser downloads them once and runs them locally.

### package.json

Node's equivalent of `Cargo.toml`. Lists the project's JavaScript/TypeScript dependencies and defines scripts like `npm run dev` and `npm run build`. Running `npm install` downloads everything listed here into a local `node_modules/` folder. This folder is never committed to git — anyone cloning the repo runs `npm install` to recreate it.

### vite.config.ts

Configuration for **Vite**, the frontend build tool. Vite does two things: runs a fast development server with hot reload so you see changes instantly without manually refreshing the browser, and bundles everything into optimized static files for production. This config file tells Vite where to find source files, how to proxy API and WebSocket requests to the Rust server during development, and where to output the final compiled build.

### src/canvas/

Everything related to the **Three.js WebGPU canvas**. Map rendering, token placement and movement, grid drawing, fog of war, lighting. This layer only knows about visual objects on a map — it has no concept of characters, game rules, or dice. It is intentionally kept system-agnostic so it works identically regardless of which game system plugin is loaded.

### src/ui/

All the **Svelte components** that make up the game interface. Character sheets, chat panel, sidebar, dice tray, scene navigator. These are normal HTML elements rendered on top of the canvas layer. Svelte is a frontend framework that compiles your component code into efficient vanilla JavaScript at build time — unlike React, there is no runtime library shipped to the browser.

### src/state/

The **shared game state** that sits between the canvas and the UI. Neither layer talks to the other directly — instead both read from and write to this shared state. When the server broadcasts a token move event over WebSocket, the state layer receives it, updates the state, and both the canvas and any relevant UI components react to the change automatically. This is what keeps the two layers decoupled.

---

## plugins/

```
plugins/
└── dnd5e/
    ├── src/
    └── package.json
```

Game system plugins live here. Each plugin is an independent JavaScript/TypeScript package that registers itself with the core application at load time. The core never hardcodes any game-system-specific logic — everything specific to a ruleset comes from a plugin.

### plugins/dnd5e/

The Dungeons and Dragons 5th Edition game system plugin. This package defines what a D&D character looks like (ability scores, hit points, spell slots, etc.), what items exist (weapons, spells, equipment), how dice rolls work in 5e, and what the character sheet UI looks like. It registers all of this with the core application via the plugin API on load.

Other game systems (Pathfinder, Call of Cthulhu, Blades in the Dark) would each live as their own folder under `plugins/` following the same pattern.

---

## docs/

```
docs/
├── REQUIREMENTS.md
├── DEV_SETUP.md
└── PROJECT_STRUCTURE.md  ← this file
```

Planning and reference documentation. Not code — just the written record of what the project is trying to do and how it is set up. New collaborators should read these before touching anything else.

**REQUIREMENTS.md** — what the system must do, organized by domain. The source of truth for scope decisions. If a feature isn't in here it isn't in scope yet.

**DEV_SETUP.md** — step by step instructions for getting a development environment running on Windows with WSL2.

**PROJECT_STRUCTURE.md** — this file.

---

## How the pieces connect

During development, two processes run simultaneously:

```
npm run dev       (inside client/)   → http://localhost:5173
cargo run         (inside server/)   → http://localhost:3000
```

Vite's dev server runs on port 5173 and serves the frontend with hot reload. It proxies any requests to `/api` or `/ws` through to the Rust server on port 3000. From the browser's perspective it all looks like one application.

For a production build:

```
npm run build     (inside client/)   → outputs to client/dist/
cargo build --release  (inside server/)  → outputs a single binary
```

The Rust binary serves the compiled frontend files from `client/dist/` as static files. Players and the GM connect to the Rust server directly — no Node.js, no npm, no Vite. Just the binary and a `.db` file per campaign world.