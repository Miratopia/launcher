# AGENTS.md — Miratopia Minecraft Launcher

## Project Overview

Miratopia is a **Minecraft launcher** built as a desktop app with **Tauri 2** (Rust backend) and **Nuxt 4** (Vue 3 frontend). It handles Microsoft/offline authentication, modpack downloading, Minecraft instance launching (Fabric, Forge, NeoForge, Quilt), and auto-updates via GitHub releases.

## Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Framework | Nuxt | 4.x |
| UI | Vue 3 (`<script setup>`) | 3.5+ |
| Desktop | Tauri | 2.x |
| Backend | Rust | stable |
| Styling | Tailwind CSS | via `@nuxtjs/tailwindcss` |
| State | Pinia | via `@pinia/nuxt` |
| Package manager | Bun | latest |
| Launcher lib | `lighty-launcher` | Git (branch `pr-fix-neoforge-loader`) |
| Credentials | `tauri-plugin-stronghold` | 2.x |

## Project Structure

```
launcher/
├── src/                        # Nuxt source (srcDir)
│   ├── app.vue                 # Root component, global event setup
│   ├── assets/css/             # Tailwind entry
│   ├── composables/            # Vue composables (Tauri events, etc.)
│   ├── layouts/                # Nuxt layouts
│   ├── pages/                  # File-based routing
│   └── stores/                 # Pinia stores
├── src-tauri/                  # Rust / Tauri backend
│   ├── src/
│   │   ├── lib.rs              # App setup, tray, plugins
│   │   ├── main.rs             # Entry point
│   │   ├── commands/           # Tauri command handlers
│   │   ├── events.rs           # EventBus → Tauri event bridge
│   │   └── types.rs            # Shared IPC payload types
│   ├── capabilities/           # Tauri permission capabilities
│   ├── Cargo.toml
│   └── tauri.conf.json
├── nuxt.config.ts
├── tailwind.config.ts
└── package.json
```

## Nuxt Conventions

### SPA Mode Only

The app runs with `ssr: false`. Never introduce server-side rendering logic, `useAsyncData` with server-side fetching, or server API routes. All data comes from Tauri `invoke()` calls or Tauri events.

### Source Directory

`srcDir` is set to `src/`. All imports using `~/` or `@/` resolve to `src/`. Always use `~/` alias for imports within the Nuxt app instead of relative paths like `../stores/`.

```typescript
// Correct
import { useConsoleStore } from '~/stores/consoleStore'

// Avoid
import { useConsoleStore } from '../stores/consoleStore'
```

### Pages & Routing

- Pages live in `src/pages/` and use Nuxt file-based routing.
- Use `<NuxtLink>` instead of `<router-link>` for navigation.
- Use `navigateTo()` for programmatic navigation.

### Composables

- Place composables in `src/composables/` with `use` prefix (e.g., `useLightyEvents.ts`).
- Nuxt auto-imports composables from this directory — no manual import needed in components.
- Composables that manage Tauri event listeners must clean up with `onUnmounted`.

### Layouts

- Default layout is in `src/layouts/default.vue`.
- Pages use the default layout unless explicitly specified with `definePageMeta({ layout: 'other' })`.

### Components

- Place components in `src/components/`. Nuxt auto-imports them.
- Use PascalCase for component filenames (e.g., `PlayerCard.vue`, `DownloadProgress.vue`).
- Use `<script setup lang="ts">` in all Vue SFCs.

## Vue / TypeScript Conventions

### Script Setup

All Vue components must use the Composition API with `<script setup lang="ts">`:

```vue
<script setup lang="ts">
const props = defineProps<{
  name: string
  count?: number
}>()

const emit = defineEmits<{
  update: [value: string]
}>()
</script>
```

### TypeScript Strictness

- Strict mode is enabled. Never use `any` unless absolutely necessary — prefer `unknown` and narrow with type guards.
- Define explicit types for Tauri `invoke()` return values and event payloads.
- Use interfaces for object shapes and `type` for unions/aliases.

### Reactivity

- Use `ref()` for primitives, `reactive()` for objects.
- Prefer `computed()` for derived state over manual watchers.
- Avoid direct mutation of Pinia state outside store actions.

## State Management (Pinia)

- Stores live in `src/stores/` with the naming pattern `{name}Store.ts`.
- Use the Options API style for Pinia stores (state/getters/actions) as established by `consoleStore.ts`.
- Pinia is auto-imported by `@pinia/nuxt` — use `defineStore()` directly without importing it.
- Each store should have a single responsibility.

## Tailwind CSS

- Entry file: `src/assets/css/tailwind.css`.
- Custom theme: `primary` color mapped to Tailwind `yellow` palette.
- Dark theme base: `bg-gray-800` + `text-gray-100` applied to `body`.
- Use Tailwind utility classes exclusively. Avoid writing custom CSS unless strictly necessary.
- Follow mobile-first responsive design when relevant (the app is desktop-only but future-proofing is good).

## Tauri / Rust Backend

### Commands

- All Tauri commands are in `src-tauri/src/commands/` and registered in `mod.rs`.
- Commands are async and return `Result<T, String>`.
- Group commands by domain: `accounts.rs` for auth, `game.rs` for launching.
- Use `State<'_, T>` for accessing managed state (e.g., `VaultState`, `EventBus`).

### IPC Pattern

Frontend-to-backend communication follows two patterns:

1. **Request/Response** — `invoke("command_name", { args })` for one-off operations.
2. **Events** — `listen("lighty://event-name", callback)` for streaming data.

Event namespace: all custom events are prefixed with `lighty://`.

| Event | Payload | Direction |
|-------|---------|-----------|
| `lighty://download-progress` | `DownloadProgressPayload` | Backend → Frontend |
| `lighty://launch-status` | `LaunchStatusPayload` | Backend → Frontend |
| `lighty://console-output` | `ConsoleLinePayload[]` | Backend → Frontend |
| `lighty://error` | `ErrorPayload` | Backend → Frontend |

### Event Bridge

`events.rs` subscribes to `lighty-launcher`'s `EventBus` and emits Tauri events. Progress updates are throttled (100ms) and console output is batched (250ms).

### Credentials & Security

- User credentials are stored in a Stronghold vault (`vault.hold`).
- Salt is stored separately in `salt.txt`.
- Key derivation uses Argon2.
- Sensitive data in memory must use `zeroize` for cleanup.
- Never log credentials, tokens, or passwords.

### Rust Style

- Use `tracing` macros (`info!`, `warn!`, `error!`) for logging, not `println!`.
- Handle errors with `Result` and `?` — avoid `.unwrap()` in production code.
- Keep command handlers thin: delegate heavy logic to dedicated modules or `lighty-launcher`.

## Development Workflow

```bash
bun install                    # Install JS dependencies
bun run install-sccache        # Install sccache for faster Rust builds
bun run tauri dev              # Start dev mode (Nuxt + Tauri)
bun run tauri:dev-fast         # Dev with sccache
bun run tauri:build-fast       # Production build with sccache
```

### Platform

- Primary target: **Windows** (MinGW-w64 toolchain via `.cargo/config.toml`).
- CI builds for macOS (aarch64, x86_64), Ubuntu, and Windows.

### Versioning

Version is synced across `package.json`, `Cargo.toml`, and `tauri.conf.json` via `scripts/bump.ts`. Never update versions manually — use the CI workflow or the bump script.

## General Rules

1. **No SSR** — This is a Tauri SPA. Never add server-side logic.
2. **Type everything** — All function params, return types, and event payloads must be typed.
3. **Clean up listeners** — Always unsubscribe Tauri event listeners in `onUnmounted`.
4. **Use auto-imports** — Nuxt auto-imports Vue APIs, composables, and components. Don't add manual imports for `ref`, `computed`, `defineStore`, etc.
5. **Prefer `~/` paths** — Use Nuxt's `~/` alias for all internal imports.
6. **Keep components small** — Extract reusable UI into `src/components/`.
7. **Pinia for shared state** — Use stores for state that crosses component boundaries.
8. **Tauri for I/O** — All file system, network, and system operations go through Tauri commands.
9. **French project context** — The team is French-speaking. Comments and commit messages may be in French or English; be consistent within a file.
