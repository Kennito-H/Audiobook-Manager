# Audiobook Manager

A desktop audiobook manager for Linux and macOS that works with your [TorBox](https://torbox.app) account: search sources, send to TorBox, download or stream, organize your library and listen with chapters and saved progress.

Built with Tauri 2 (Rust) and React + TypeScript. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the design.

> Status: early development — the app shell is in place; features are being built milestone by milestone.

## Requirements

|                        | macOS                                                                    | Linux                                                                                                 |
| ---------------------- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| System tools           | Xcode Command Line Tools: `xcode-select --install`                     | `sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf` (Debian/Ubuntu) |
| Rust 1.85+ (stable)    | [rustup](https://rustup.rs)                                               | [rustup](https://rustup.rs)                                                                            |
| Node.js (LTS or newer) | `brew install node` or [nodejs.org](https://nodejs.org)                 | your package manager or [nodejs.org](https://nodejs.org)                                               |
| pnpm                   | `brew install pnpm`, or `npm install -g corepack && corepack enable` | `npm install -g corepack && corepack enable`                                                        |

You don't need to install the Tauri CLI or SQLite separately. `pnpm install` gets the Tauri CLI, and SQLite is built into the app. Once playback is added, you'll also need libmpv (`brew install mpv` on macOS, `libmpv-dev` on Linux).

## Opening the app

First, from the project folder:

```sh
pnpm install
```

### Run it in development mode

```sh
pnpm tauri dev
```

The first launch compiles the Rust code, so it takes a few minutes. Later launches are fast. The app window opens on its own. Changes to the UI reload instantly, and changes to the Rust code rebuild and restart the app. To quit, close the window or press `Ctrl+C` in the terminal.

`tauri dev` is a pnpm command. Don't run it through Homebrew (`brew install tauri dev` won't work).

### Build and open a standalone app

```sh
pnpm tauri build
```

The build output goes to `src-tauri/target/release/bundle/`:

- **macOS:** `macos/Audiobook Manager.app` (drag it into Applications) and `dmg/Audiobook Manager_<version>_<arch>.dmg`.
- **Linux:** `appimage/*.AppImage` (run `chmod +x` on it, then run it) and `deb/*.deb` (install with `sudo apt install ./<file>.deb`).

**macOS Gatekeeper:** builds aren't signed yet. If macOS says the app "can't be opened" (common with a `.dmg` downloaded from CI), go to **System Settings → Privacy & Security** and click **Open Anyway** (on macOS 14 and earlier, you can instead right-click the app and choose **Open**). Or run:

```sh
xattr -dr com.apple.quarantine "/Applications/Audiobook Manager.app"
```

## Development checks

```sh
pnpm typecheck && pnpm lint && pnpm test
cd src-tauri && cargo clippy --all-targets -- -D warnings && cargo test
```
