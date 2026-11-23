# Security Policy

## Supported versions

Only the [latest release](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest)
receives fixes.

## Reporting a vulnerability

Please **do not open a public issue**. Report it privately through GitHub:
[**Report a vulnerability**](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/security/advisories/new).

Include what you found, how to reproduce it and the affected version. You should get a
first answer within a week. Once fixed, the issue will be disclosed in the release notes
with credit, unless you prefer to stay anonymous.

## What the app does (for reviewers)

- No network access, no telemetry.
- Writes only to `HKCU\Software\WirelessBatteryMonitor` (overlay position) and, when
  *Start with Windows* is enabled, `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
- Runs with normal user privileges (no elevation).
- Release binaries are built by GitHub Actions from tagged commits and carry a build
  provenance attestation (`gh attestation verify`).
