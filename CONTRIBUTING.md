# Contributing

Thanks for taking the time to contribute!

## Antes de empezar / Before you start

- Open an issue first for anything larger than a small fix. That avoids
  duplicated work and lets us agree on the approach.
- For security problems, **do not** open a public issue: contact the
  maintainer directly.

## Development setup

1. Install [Rust](https://rustup.rs) (MSVC toolchain).
2. Clone the repository.
3. Build and run:

   ```powershell
   cargo run --release
   ```

4. To check which devices are detected without opening the overlay:

   ```powershell
   cargo run --release -- --list | more
   ```

## Pull requests

- Keep changes focused; one logical change per PR.
- Match the existing style: everything in `src/main.rs`, Spanish UI strings.
  Run `cargo test` before opening the PR.
- Explain **why** the change is needed in the PR description, and mention how
  you tested it (which Windows version and devices).
- If you touch Win32 interop or taskbar embedding, note that the window
  hierarchy can differ between Windows builds — include a screenshot or a
  short recording when possible.

## Reporting bugs

Please include:

- Windows version (`winver`).
- Device brand and model.
- Output of `BatteryMonitor.exe --list`.
- Steps to reproduce and what you expected to happen.

## License

By contributing you agree that your contributions are licensed under the
[MIT License](LICENSE).
