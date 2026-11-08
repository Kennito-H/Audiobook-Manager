For this project we are working to create a desktop audiobook manager that pairs up with your TorBox account (TorBox cloud API only, no self-hosted server).

Target platforms: Linux and macOS. Full architecture plan and milestones: see "Architecture" below; the original plan lives at ~/.claude/plans/read-the-claude-md-and-reflective-squid.md.

## Stack (decided)
- Tauri 2 shell. Rust core in `src-tauri/`, React 19 + TypeScript + Vite UI in `src/`.
- UI: Tailwind v4 + shadcn/ui (`src/components/ui`), TanStack Query for data from Rust, Zustand for UI/player state, react-router (MemoryRouter).
- Rust ↔ TS: tauri-specta. Commands get `#[tauri::command] #[specta::specta]` and are registered in `specta_builder()` in `src-tauri/src/lib.rs`. `src/lib/bindings.ts` is GENERATED (debug launch or `cargo test`) — never edit it by hand; commit it after regenerating. specta/tauri-specta are pinned RC versions that must match each other.
- DB: SQLite via sqlx; migrations in `src-tauri/migrations/` (one per milestone feature; never edit an applied migration). DB file is `<app data dir>/library.db`.
- Errors: commands return `AppResult<T>`; `AppError` serializes to `{ kind, message }`.
- Planned: libmpv (`libmpv2`) for playback/streaming, lofty + mp4ameta for tags/chapters, keyring for the TorBox API key (never store secrets in the DB), Audnexus/Open Library metadata.

## Search policy
- Built-in sources are legal ones only (LibriVox, Internet Archive). They are ordinary rows in `search_sources` and can be disabled, removed or reordered.
- Users add their own sources in Settings as Torznab/Newznab or RSS/OPDS feed URLs. Do not ship scrapers for specific sites. Manual magnet/NZB/.torrent add always works.

## Milestones
1. Scaffold (done) · 2. TorBox client + Settings · 3. Acquire + Downloads · 3b. Search · 4. Library · 5. Player · 6. Streaming · 7. Polish

## Commands
- `pnpm tauri dev` — run the app
- `pnpm typecheck && pnpm lint && pnpm test` — frontend checks
- `cd src-tauri && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` — Rust checks
- CI (`.github/workflows/build.yml`) runs all of the above on ubuntu-22.04 and macos-latest and builds bundles.
