# Requirements

This document defines what the system must do. It is the source of truth for scope decisions. Stories and milestones are derived from this document, not the other way around.

Requirements are organized by domain. Core requirements apply regardless of game system. Plugin requirements apply only to a specific game system implementation.

---

## Core

Core requirements are system-wide concerns that any game system plugin will depend on. Nothing here knows what a character is, what a dice notation means, or what rules govern a specific game.

### Server

**S-01** — The server must compile to a single self-contained binary with no external runtime dependencies.

**S-02** — The server must serve the compiled frontend as static files from the same binary.

**S-03** — The server must accept WebSocket connections from multiple simultaneous clients.

**S-04** — The server must broadcast game events received from any client to all connected clients.

**S-05** — The server must persist game events to the database alongside broadcasting them so state survives a reload.

**S-06** — The server must expose a REST API for structured reads and writes that do not require real-time broadcast.

**S-07** — The server must serve user-uploaded assets (maps, token images, audio) as static files.

**S-08** — The server must accept a world file path as a startup argument, loading the specified SQLite database on launch.

**S-09** — The server must be reachable over the internet via port forwarding without additional infrastructure.

**S-10** — The server must support a configurable port number.

### Database

**D-01** — Each campaign world must be stored as a single SQLite file.

**D-02** — The database schema must be system-agnostic. Game-system-specific fields must be stored as JSON blobs that the core never inspects or validates.

**D-03** — The database must include the following core tables: `actors`, `items`, `scenes`, `messages`, `documents`.

**D-04** — The database must track which game system plugin a world was created with.

**D-05** — The database must support schema migrations so existing world files can be upgraded between versions without data loss.

### WebSocket Protocol

**W-01** — All real-time game events must be transmitted as JSON messages over WebSocket.

**W-02** — Every message must include a `type` field that identifies the event kind.

**W-03** — The protocol must support the following core event types: `token.move`, `token.add`, `token.remove`, `fog.update`, `scene.change`, `chat.message`, `dice.result`.

**W-04** — The protocol must be extensible — plugins must be able to register and transmit custom event types without modifying core.

**W-05** — The server must silently ignore unknown event types rather than erroring, to support plugin messages arriving before plugin registration.

### Canvas

**C-01** — The canvas must render using a WebGPU renderer with automatic fallback to WebGL2 for unsupported browsers.

**C-02** — The canvas must display a scene as a tile-based grid.

**C-03** — The canvas must support placing tokens on the grid.

**C-04** — The canvas must support moving tokens by drag and drop.

**C-05** — Token movement must be broadcast to all connected clients and reflected on their canvas in real time.

**C-06** — The canvas must support loading a map image as the scene background.

**C-07** — The canvas must support panning and zooming.

**C-08** — The canvas must render tokens above the map layer and UI elements above the token layer.

**C-09** — The canvas must never contain game-system-specific logic or rendering.

### UI

**U-01** — The UI must be implemented as HTML/CSS overlaid on top of the canvas, not rendered within it.

**U-02** — The UI must include a chat panel displaying messages from all connected clients in order.

**U-03** — The UI must include a sidebar for navigating scenes, actors, and items.

**U-04** — The UI and canvas must communicate only through shared game state, never directly.

**U-05** — The UI must be responsive to the game state updating in real time without requiring a page reload.

### Plugin System

**P-01** — The core application must expose a plugin registration API that game system plugins call on load.

**P-02** — The registration API must accept the following from a plugin: actor type schemas, item type schemas, character sheet UI components, and a dice resolver.

**P-03** — Plugin-registered UI components must be rendered by the core application without the core knowing their internal implementation.

**P-04** — The core must never hardcode any game-system-specific logic, schema, or UI.

**P-05** — The plugin API must be versioned so that plugins can declare which API version they target.

**P-06** — Plugins must be loadable as JavaScript modules without requiring a server restart.

**P-07** — The active game system plugin must be stored in the world database and loaded automatically when that world is opened.

---

## Plugins

Plugin requirements are specific to a game system implementation. They depend on the core plugin system being stable.

### dnd5e

**5E-01** — The plugin must register an actor type of `character` with a schema covering: ability scores, saving throws, skills, hit points, armor class, speed, proficiency bonus, class and level, and spell slots.

**5E-02** — The plugin must register an actor type of `npc` with a schema covering: hit points, armor class, speed, challenge rating, and actions.

**5E-03** — The plugin must register the following item types: `weapon`, `spell`, `feature`, `equipment`, `consumable`.

**5E-04** — The plugin must provide a read-only character sheet UI component for the `character` actor type displaying all schema fields.

**5E-05** — The plugin must provide a read-only stat block UI component for the `npc` actor type.

**5E-06** — The plugin must register a dice resolver that handles standard 5e notation: `1d20`, `2d6+3`, advantage (`2d20kh1`), disadvantage (`2d20kl1`).

**5E-07** — Dice results must include the individual die values, any modifiers, and the total.

**5E-08** — Dice results must be broadcast to all clients as a `dice.result` event and displayed in the chat panel.

**5E-09** — The plugin must provide a dice tray UI component from which any client can initiate a roll.
