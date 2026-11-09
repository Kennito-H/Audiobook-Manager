# Audiobook Manager

A desktop audiobook manager for Linux and macOS that works with your [TorBox](https://torbox.app) account: search sources, send to TorBox, download or stream, organize your library and listen with chapters and saved progress.

Built with Tauri 2 (Rust) and React + TypeScript. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the design.

> Status: early development — the app shell is in place; features are being built milestone by milestone.

## Development

Requirements: Rust (stable), Node.js, pnpm, and on Linux the WebKitGTK 4.1 dev packages (plus libmpv once playback lands).

```sh
pnpm install
pnpm tauri dev
```

Checks:

```sh
pnpm typecheck && pnpm lint && pnpm test
cd src-tauri && cargo clippy --all-targets -- -D warnings && cargo test
```
