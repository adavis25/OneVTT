# Dev environment setup

This project runs on a WSL2 + Rust + Node stack. Follow this guide to get your environment ready before touching the repo.

---

## Prerequisites

You'll need Windows 10 (build 19041+) or Windows 11. Everything else gets installed through this guide.

---

## 1. Install WSL2

Open PowerShell as Administrator and run:

```powershell
wsl --install
```

This installs WSL2 with Ubuntu by default. Restart your machine when prompted.

After restart, verify you're on WSL2 (not WSL1):

```powershell
wsl --list --verbose
```

The `VERSION` column should show `2` next to your Ubuntu distro. If it shows `1`, run:

```powershell
wsl --set-version Ubuntu 2
```

---

## 2. Keep project files inside WSL

This is important. Always work from inside the WSL file system, not from a mounted Windows drive.

```bash
# Correct — native WSL file system, fast
~/projects/vtt/

# Wrong — mounted Windows drive, slow builds and broken file watching
/mnt/c/Users/yourname/projects/vtt/
```

Cargo compile times on `/mnt/c/` can be 3–5x slower, and Vite's file watcher has reliability issues on mounted drives.

---

## 3. Install Windows Terminal

Install [Windows Terminal](https://aka.ms/terminal) from the Microsoft Store if you don't have it. Set your default profile to Ubuntu so it opens straight into WSL.

---

## 4. Install system dependencies

Open Ubuntu in Windows Terminal and run:

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential pkg-config libssl-dev
```

These are C build tools and SSL headers that Rust crates commonly need. Better to have them now than hit a confusing error mid-build later.

---

## 5. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Accept the default installation when prompted. Then reload your shell environment:

```bash
source ~/.cargo/env
```

---

## 6. Install Node via nvm

We use `nvm` rather than installing Node directly so the version is consistent across machines.

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
```

Restart your terminal (or run `source ~/.bashrc`), then install and pin Node 22:

```bash
nvm install 22
nvm use 22
nvm alias default 22
```

---

## 7. Verify everything

```bash
cargo --version    # cargo 1.x.x
rustc --version    # rustc 1.x.x
node --version     # v22.x.x
npm --version      # 10.x.x
```

All four should return version numbers without errors.

---

## 8. VS Code setup

Install the [WSL extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) for VS Code. Then open the project from inside WSL:

```bash
cd ~/projects/vtt
code .
```

This opens VS Code on the Windows side but everything — terminal, file system, language servers — runs inside WSL. Install the following extensions while connected to WSL (they'll install into the WSL environment automatically):

| Extension | ID | Purpose |
|---|---|---|
| Rust Analyzer | `rust-lang.rust-analyzer` | Rust intellisense and type checking |
| Svelte | `svelte.svelte-vscode` | Svelte language support |
| Even Better TOML | `tamasfe.even-better-toml` | Cargo.toml syntax highlighting |
| ESLint | `dbaeumer.vscode-eslint` | TypeScript linting |
| TypeScript Vue Plugin | `vue.volar` | Better TS support in .svelte files |
| Thunder Client | `rangav.vscode-thunder-client` | Test your REST API endpoints without leaving VS Code |
| GitLens | `eamodio.gitlens` | Enhanced git history and blame |
| Error Lens | `usernamehw.errorlens` | Inline error messages in the editor |

---

## 9. Git Repo Setup

I suggest using [GitHub Desktop](https://desktop.github.com/download/) for version control.  Once its installed and logged in you should be able to clone the repo and then set the local directory path to the WSL directory and then where you want it from there. 
```
\\wsl.localhost\Ubuntu\home\your-username
```

---

## 10. Dev workflow

During development, run the backend and frontend simultaneously in two terminal tabs.

**Terminal 1 — Rust backend (port 3000):**
```bash
cd server
cargo run
```

**Terminal 2 — Vite frontend (port 5173):**
```bash
cd client
npm run dev
```

Vite proxies `/api` and `/ws` requests to the Rust server automatically, so both feel like one app. Edit frontend code and it hot-reloads instantly. Edit backend code and re-run `cargo run`.

---

## Troubleshooting

**`cargo` command not found after install**
Run `source ~/.cargo/env` or restart your terminal. If it still doesn't work, check that `~/.cargo/bin` is in your `PATH`.

**`nvm` command not found after install**
Run `source ~/.bashrc` or restart your terminal. The install script adds nvm to your shell profile but the current session won't pick it up until reloaded.

**VS Code extensions installing to Windows instead of WSL**
Make sure you opened VS Code via `code .` from inside a WSL terminal, not by launching it from Windows. The title bar should show `[WSL: Ubuntu]` when connected correctly.

**Slow build times**
Double-check your project is not under `/mnt/c/`. Run `pwd` — if the path starts with `/mnt/`, move the project to `~/projects/` inside WSL.