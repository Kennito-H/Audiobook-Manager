# Architecture

Audiobook Manager is a desktop app for **Linux and macOS** that works with a **TorBox** account. It finds audiobooks, sends them to TorBox, then downloads or streams them, keeps them organized in a local library, and plays them with chapters and saved progress. It only talks to TorBox's hosted cloud API. There is no self-hosted server.

> **Status:** milestone 1 (scaffold) is done. Sections marked _planned_ describe the design for later milestones and may change as they are built.

## Tech stack

| Layer | Choice | Why |
|---|---|---|
| App shell | **Tauri 2** | Small native app for Linux and macOS, using the system webview |
| UI | **React 19 + TypeScript + Vite** | Widely used, transferable frontend skills |
| UI kit and styling | **Tailwind v4 + shadcn/ui** | Fast to build with; components live in `src/components/ui` |
| Server-state cache | **TanStack Query** | Caches and polls data from Rust commands |
| UI state | **Zustand** | Player and UI state |
| Routing | **react-router** (`MemoryRouter`) | Desktop app, so no URL bar |
| Rust ↔ TS types | **tauri-specta** | Generates `src/lib/bindings.ts` from the Rust commands |
| HTTP | **reqwest + tokio** _(planned)_ | TorBox API, metadata, downloads |
| Database | **SQLite via sqlx**, with migrations | Library, progress, download queue, sources |
| Audio playback | **libmpv (`libmpv2` crate)** _(planned)_ | Plays m4b and mp3, reads chapters, streams from HTTP, changes speed without changing pitch. Avoids the Linux webview's AAC codec gaps |
| Tags and chapters | **lofty**, **mp4ameta** _(planned)_ | Used when scanning files |
| Secrets | **keyring** crate _(planned)_ | Keeps API keys in macOS Keychain or Linux Secret Service. Never stored in the database |
| Metadata | **Audnexus**, with **Open Library** as fallback _(planned)_ | Covers, series, narrators |
| Testing | `cargo test` (+ `wiremock` _(planned)_), **Vitest** + Testing Library | |
| Tooling | pnpm, rustfmt, clippy, ESLint, Prettier, GitHub Actions | CI builds Linux deb/AppImage and macOS dmg |

## System overview

```
┌──────────────── React/TS UI (webview) ────────────────┐
│ Pages: Library · Search · TorBox · Downloads ·        │
│        Settings · Player bar                          │
│ TanStack Query ◄──── typed commands / events ─────┐   │
└───────────────────────────────────────────────────│───┘
                       Tauri IPC (commands + events)│
┌──────────────── Rust core (src-tauri) ────────────▼───┐
│ search/    SearchProvider trait: LibriVox, Internet   │
│            Archive, Torznab/Newznab, RSS/OPDS         │
│ torbox/    API client: add torrent/usenet/web job,    │
│            list, cache check, request download link   │
│ downloads/ queue, parallel workers, HTTP Range resume,│
│            progress events → UI                       │
│ library/   folder scanner + file watcher, tag/chapter │
│            parsing, metadata matching, rename rules   │
│ player/    libmpv: local file OR TorBox stream URL,   │
│            chapters, speed, sleep timer, saves        │
│            position to the DB                         │
│ db/        sqlx SQLite + migrations           ✔ built │
│ secrets/   keyring for API keys                       │
└───────────────────────────────────────────────────────┘
        │ HTTPS                          │ HTTPS
   api.torbox.app/v1          Audnexus / Open Library / search sources
```

**How the UI talks to Rust.** Every Rust command has both `#[tauri::command]` and `#[specta::specta]`, and is registered in `specta_builder()` in `src-tauri/src/lib.rs`. Debug builds and `cargo test` regenerate `src/lib/bindings.ts`. The UI calls `commands.xxx()`, which returns `{ status: "ok", data } | { status: "error", error }`. Errors come from `AppError` and are serialized as `{ kind, message }`. CI fails if the bindings checked into the repo are out of date.

**Long-running work** (polling, downloads, playback position) reports to the UI through Tauri events, not polling from the UI.

## Main flows

### 0. Search _(milestone 3b)_
The Search page sends the query to every enabled source at the same time. It merges and de-duplicates the results (title, size, seeders, source) and shows them. **Get** passes the selected result to step 1.

- **Built-in sources:** LibriVox and Internet Archive. These are legal, public-domain books.
- **Custom sources:** added in Settings → Sources with a name, a type (**Torznab/Newznab** or **RSS/OPDS**), a URL, and an optional API key. Torznab/Newznab covers Prowlarr, Jackett and sites that offer it.
- **Every source is a row in `search_sources`.** Built-in ones can be turned off, deleted or reordered, and the source at the top is the default.
- **No site-specific scrapers are shipped.** Users choose which sources they add.
- **Manual add always works:** paste a magnet link or NZB, or drop a .torrent file.

### 1. Acquire _(milestones 2–3)_
`torbox::create_*` sends the job to TorBox. The core polls the user's TorBox list until the job finishes. The user then chooses:

- **Download:** the job joins the download queue, the file is saved to the library folder, and the library rescans it.
- **Stream:** go to step 2.

### 2. Stream _(milestone 6)_
`torbox::request_link` gets an HTTPS URL, and `player::load(url)` plays it in mpv. Progress is saved against the TorBox file ID, and moves to the local copy if the book is downloaded later.

### 3. Library _(milestone 4)_
Watched folders are scanned. Each file's tags and chapters are read, then matched to metadata, by ASIN from the tags or by a fuzzy title/author search. The results are saved to the database.

### 4. Playback _(milestone 5)_
The player sends events (position, chapter, state) that drive the player bar. Position is saved every few seconds, so a book resumes where you stopped.

## Data model

Migrations live in `src-tauri/migrations/`. Each feature adds its own migration, and a migration that has already been applied is never edited.

| Table | Status | Purpose |
|---|---|---|
| `settings` | ✔ built (`0001_init.sql`) | Key/value app settings (no secrets) |
| `books`, `authors`, `narrators`, `series` | planned | Library catalogue (series includes position) |
| `book_authors`, `book_narrators` | planned | Many-to-many links |
| `files` | planned | Local path or TorBox file ID, duration |
| `chapters` | planned | Per-file chapter list |
| `progress` | planned | Book, position, updated_at |
| `torbox_items` | planned | Links a TorBox ID to a book |
| `downloads` | planned | Status, bytes done, error |
| `search_sources` | planned | Name, kind, URL, keyring reference for the API key, enabled, order |

The database file is `<app data dir>/library.db`, for example `~/.local/share/com.audiobookmanager.app/library.db` on Linux.

## Project layout

```
src/                     React app
  App.tsx                shell: sidebar + routes + player bar
  pages/                 one component per page
  components/            shared components (ui/ = shadcn)
  stores/                Zustand stores
  lib/bindings.ts        GENERATED by tauri-specta (do not edit)
src-tauri/
  src/lib.rs             app setup, command registration, binding export
  src/commands.rs        Tauri commands
  src/error.rs           AppError → { kind, message }
  src/db/                pool + migrations runner
  migrations/            SQL migrations
  tauri.conf.json        app config (window, bundle targets)
.github/workflows/       CI: checks + bundles on Linux and macOS
```

Later modules (`torbox/`, `downloads/`, `search/`, `library/`, `player/`, `secrets/`) are added under `src-tauri/src/` by the milestones that need them.

## Milestones

| # | Milestone | Status |
|---|---|---|
| 1 | Scaffold: Tauri + React/TS, Tailwind/shadcn, tauri-specta, sqlx, CI | ✔ done |
| 2 | TorBox client + Settings: API key in keyring, "test connection", list of TorBox items | next |
| 3 | Acquire + Downloads: add magnet/torrent/NZB, poll status, resumable download queue | |
| 3b | Search: `SearchProvider` trait, LibriVox/IA, Torznab/Newznab + RSS/OPDS, Sources settings | |
| 4 | Library: folder scan, tags/chapters, grid/list views, metadata matching | |
| 5 | Player: libmpv playback, chapters, speed, sleep timer, resume | |
| 6 | Streaming: play TorBox links directly, link progress to the local copy | |
| 7 | Polish: rename rules, search/filter, keyboard shortcuts, packaging and signing | |

## Risks to check early

- **Shipping libmpv on macOS:** the app must include the mpv libraries. Prototype this in CI when the player lands. Fallback: play audio in the webview on macOS, which handles AAC.
- **TorBox API details:** check endpoints, rate limits and any search API against the current docs in milestone 2. If TorBox has its own search, it can become another `SearchProvider`.
- **Linux webview:** WebKitGTK only renders the UI. Audio goes through mpv, so the webview's codec gaps don't matter.
- **tauri-specta and specta are release candidates** pinned to matching versions (`=2.0.0-rc.26`). Upgrade both together.

## Testing

- `pnpm typecheck && pnpm lint && pnpm test`: frontend checks.
- `cd src-tauri && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`: Rust checks, including the migration test and the bindings-export test.
- Planned: run the TorBox client against `wiremock` fixtures, and the scanner against sample mp3/m4b files in `src-tauri/tests/fixtures`.
- Manual end-to-end with a real API key once the features exist: add an item, download it, check it appears in the library, play and pause, restart the app and resume, then stream a second item.
