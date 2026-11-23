# Changelog

All notable changes are documented here. Format based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow [SemVer](https://semver.org).

## [1.1.0]

### Added
- Windows notification when a device drops to 20 % or below (once per device, re-armed after it charges or reconnects).
- Menu and tooltip in English or Spanish, following the Windows display language (previously Spanish only).
- Live light/dark theme switching, no restart needed.
- Release builds published automatically with a portable ZIP, `SHA256SUMS.txt` and build provenance attestation.

### Fixed
- Two identical devices on the same protocol (e.g. two "Xbox Wireless Controller") were merged into one.
- `--list` printed nothing when run directly from a terminal (without `| more`).

### Changed
- The executable is now `WirelessBatteryMonitor.exe` (was `BatteryMonitor.exe`).
  **Upgrading from 1.0.0 with *Start with Windows* on:** exit the old version, delete it,
  start the new one and tick *Start with Windows* again so the entry points to the new file.

## [1.0.0]

- First public release.

[1.1.0]: https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/tag/v1.1.0
[1.0.0]: https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/tag/1.0.0
