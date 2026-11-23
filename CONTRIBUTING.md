# Contributing

Thanks for helping! There are ways to contribute for everyone, not only Rust developers.
*(Español: puedes escribir issues y PRs en español o en inglés.)*

## No code needed

- **Report your device** — the most valuable contribution right now. Use the
  [device report form](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml),
  whether the device works or not. Reports become the [compatibility list](COMPATIBILITY.md).
- **Test on Windows 10**, a vertical/top taskbar, multiple monitors or high DPI and tell us what happens.
- **Translate** the menu: the UI has only five strings (see `tr(...)` in `src/main.rs`).
  Open an issue with the translations if you don't want to touch code.
- **Improve the docs** — typos and unclear sentences count.

## Development setup

1. Install [Rust](https://rustup.rs) (MSVC toolchain, Windows only).
2. Clone the repository and run:

   ```powershell
   cargo run --release              # start the overlay
   cargo run --release -- --list    # print detected devices and exit
   cargo test
   cargo clippy --all-targets -- -D warnings
   ```

Only one instance can run at a time: exit any running copy (tray icon → Exit) before
starting your build.

## Code layout

Everything lives in `src/main.rs`, in two halves:

- **Bluetooth** — `read_all()` gathers `Battery` entries from `read_classic()`
  (PnP `DEVPKEY_Bluetooth_Battery`) and `read_low_energy()` (GATT Battery Service).
  It runs on a worker thread and must stay free of UI code.
- **UI** — plain Win32: a hidden host window (timers, tray icon, menu) and the
  overlay window reparented into the taskbar. `wndproc` handles all messages.

Keep it small: no new dependencies or UI frameworks without discussing it in an
issue first. The release binary size (~300 KB) is a feature.

## Pull requests

- Open an issue first for anything larger than a small fix, to agree on the approach.
- One logical change per PR. Explain **why**, and how you tested it
  (Windows version, devices).
- User-visible strings go through `tr("English", "Español")`.
- If you touch the taskbar embedding or Win32 interop, add a screenshot or short
  recording — the window hierarchy differs between Windows builds.
- CI must pass (`cargo test`, `cargo clippy -D warnings`).

## Releases (maintainers)

1. Bump `version` in `Cargo.toml` and `app.manifest`, and add a section to `CHANGELOG.md`.
2. Commit, then tag and push: `git tag v1.2.0 && git push origin v1.2.0`.
3. The [release workflow](.github/workflows/release.yml) builds, packages and publishes
   the release with checksums and the changelog section as notes.

Versioning follows [SemVer](https://semver.org): new features → minor, fixes → patch.

## Reporting bugs

Use the [bug report form](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=bug_report.yml).
For security problems, see [SECURITY.md](SECURITY.md) — do not open a public issue.

## License

By contributing you agree that your contributions are licensed under the [MIT License](LICENSE).
