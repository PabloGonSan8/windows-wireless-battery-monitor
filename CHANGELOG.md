# Changelog

All notable changes are documented here. Format based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow [SemVer](https://semver.org).

## [1.1.1]

### Fixed
- *Start with Windows* silently stopped working after the executable was moved, renamed or
  replaced (e.g. upgrading from 1.0.0's `BatteryMonitor.exe`): the startup entry kept pointing
  to the old file. The app now re-points the entry to itself every time it starts, so opening
  the new version once is enough — no need to untick and tick the option again.

### Changed
- Dependency updates: `windows-registry` 0.100, GitHub Actions `checkout` v7,
  `upload-artifact` v7, `attest-build-provenance` v4.

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

[1.1.1]: https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/tag/v1.1.1
[1.1.0]: https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/tag/1.1.0
[1.0.0]: https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/tag/1.0.0
