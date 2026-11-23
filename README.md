<div align="center">

# Wireless Battery Monitor

**The battery level of your Bluetooth mouse, keyboard, headphones and controllers —
always visible in the Windows taskbar.**

[![Latest release](https://img.shields.io/github/v/release/PabloGonSan8/windows-wireless-battery-monitor?label=download&color=2ea44f)](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/PabloGonSan8/windows-wireless-battery-monitor/total)](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases)
[![Build](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/actions/workflows/build.yml/badge.svg)](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/actions/workflows/build.yml)
![Windows 10 / 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4)
![Rust](https://img.shields.io/badge/made%20with-Rust-orange)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](LICENSE)

![Wireless Battery Monitor in the Windows 11 taskbar](docs/screenshot.png)

<sub>Example values: mouse 87 %, keyboard 64 %, headphones 41 % and a gamepad at 14 % (red: below 20 %).</sub>

### [⬇ Download for Windows](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest)

Single ~300 KB `.exe` · no installer · no background service · no telemetry

[Español](README.es.md)

</div>

## Why?

Windows already knows the battery level of many Bluetooth devices, but it hides it
in *Settings → Bluetooth & devices*. You only find out your mouse is dead when it
stops moving.

Wireless Battery Monitor puts every connected device's battery right in the
taskbar, next to the clock, so you can see it at a glance — no clicks, no
flyouts, no vendor app per brand.

## Features

- 🔋 **All your devices at once**, inline in the taskbar (not hidden behind a tray icon).
- 🎧 **Bluetooth Classic** (Hands-Free profile) and **Bluetooth Low Energy** (GATT Battery Service).
- 🖱️ **Device icons** for mouse, keyboard, headphones and gamepad.
- 🔴 **Low-battery warning**: the level turns red at 20 % or below, and a Windows
  notification is shown once when a device drops there.
- 🌗 Follows the Windows **light/dark theme**, including live switching.
- ↔️ **Drag to reposition**; the position is remembered.
- 🚀 Optional **Start with Windows** from the menu.
- 🌍 Menu in **English or Spanish**, following the Windows display language.
- 🪶 **Tiny**: one native Rust executable, ~300 KB on disk, ~5 MB of private memory
  (measured on Windows 11). Survives `explorer.exe` restarts.

## Quick start

1. Download `WirelessBatteryMonitor.exe` (or the portable `.zip`) from the
   [latest release](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest).
2. Put it anywhere (for example `C:\Tools\`) and double-click it.
3. Your connected devices appear in the taskbar, left of the tray icons.
   Right-click them → **Start with Windows** to keep it running after a reboot.

> [!NOTE]
> **"Windows protected your PC"?** The executable is not code-signed yet (signing
> certificates cost money; it's on the [roadmap](ROADMAP.md)). Click
> **More info → Run anyway**. Every release is built by
> [GitHub Actions](.github/workflows/release.yml) from the public source and ships
> with a `SHA256SUMS.txt` and a build provenance attestation so you can verify it
> (see [FAQ](#faq)).

### Using it

| Action | What happens |
| --- | --- |
| Hover the overlay | Tooltip with the full device names and levels |
| Left-drag the overlay | Move it horizontally (position is saved) |
| Right-click the overlay or tray icon | Menu: **Refresh**, **Start with Windows**, **Exit** |

The levels refresh every 30 seconds.

## Will it work with my device?

**Rule of thumb:** if Windows shows a battery percentage for the device under
*Settings → Bluetooth & devices*, Wireless Battery Monitor will usually show it too.
It reads the same standard sources:

- Classic devices that report battery through the **Hands-Free profile** (most headsets/earbuds).
- LE devices that implement the standard **GATT Battery Service** (many mice, keyboards, controllers).

Devices that only report battery through a vendor app or a USB dongle (for
example Logitech Unifying/Bolt receivers) are not visible to Windows and won't appear.
AirPods are a known example of a device that does not expose its level to Windows.

📋 See the community-maintained **[compatibility list](COMPATIBILITY.md)** — and
please **[report your device](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml)**,
whether it works or not. No programming knowledge needed; it takes two minutes.

## FAQ

<details>
<summary><b>My device is connected but doesn't show up.</b></summary>

Open a terminal in the folder of the executable and run:

```powershell
.\WirelessBatteryMonitor.exe --list | more
```

It prints every device and level the app can read. If your device is not in the
list, Windows itself is not receiving a battery level from it (check *Settings →
Bluetooth & devices*). Please [report it](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml)
so it gets added to the compatibility list.
</details>

<details>
<summary><b>How is this different from the battery shown in Windows Settings?</b></summary>

Same data, different place: Settings needs several clicks and shows one page at a
time. This app keeps every connected device visible in the taskbar all the time and
warns you when one gets low.
</details>

<details>
<summary><b>Is it safe? My antivirus / SmartScreen complains.</b></summary>

The app is unsigned, so SmartScreen warns about it until it gains reputation.
It makes **no network connections**, collects nothing, and the whole source is a
single file: [`src/main.rs`](src/main.rs). Releases are built by GitHub Actions; you can check
the download with:

```powershell
Get-FileHash .\WirelessBatteryMonitor.exe -Algorithm SHA256   # compare with SHA256SUMS.txt
gh attestation verify .\WirelessBatteryMonitor.exe -R PabloGonSan8/windows-wireless-battery-monitor
```

If an antivirus flags it, please open an issue with the product name and detection.
</details>

<details>
<summary><b>Does it drain my devices' battery?</b></summary>

It reads the level once every 30 seconds from devices that are already
connected. It does not keep extra connections open. If you notice an effect on a
specific device, please open an issue.
</details>

<details>
<summary><b>Does it work on Windows 10? With the taskbar on top or on the side?</b></summary>

It is developed and tested on Windows 11 with the taskbar at the bottom. Windows 10
(2004+) and other taskbar positions are **untested** — reports are very welcome.
</details>

<details>
<summary><b>How do I uninstall it?</b></summary>

1. Right-click the overlay and untick **Start with Windows**.
2. Click **Exit** and delete the `.exe`.
3. Optionally delete the registry key `HKCU\Software\WirelessBatteryMonitor` (it only stores the overlay position).
</details>

<details>
<summary><b>Can I change the 20 % threshold, or hide a device?</b></summary>

Not yet. Both are on the [roadmap](ROADMAP.md) — upvote or comment on the issues there.
</details>

## Roadmap

Planned work lives in [ROADMAP.md](ROADMAP.md). Highlights: configurable low-battery
threshold, hiding devices, code signing, `winget`/Scoop packages, and verified
Windows 10 support.

## For developers

### Build from source

Requires [Rust](https://rustup.rs) with the MSVC toolchain.

```powershell
git clone https://github.com/PabloGonSan8/windows-wireless-battery-monitor
cd windows-wireless-battery-monitor
cargo run --release              # starts the overlay
cargo run --release -- --list    # prints detected devices and exits
cargo test
```

The binary is `target\release\WirelessBatteryMonitor.exe` and has no runtime dependencies.

### How it works

1. Enumerates connected **Bluetooth Classic** devices and matches them with PnP
   nodes that publish `DEVPKEY_Bluetooth_Battery`
   (`{104EA319-6EE2-4701-BD47-8DDBF425BBE5} 2`).
2. Enumerates connected **Bluetooth LE** devices, opens the GATT **Battery Service**
   (`0x180F`) and reads the **Battery Level** characteristic (`0x2A19`).
3. Repeats every 30 seconds on a worker thread and posts the results to the UI thread.

To sit on the Windows 11 taskbar — which draws above every topmost window — the
overlay is created as a layered popup and then reparented into `Shell_TrayWnd`
with `SetParent`, using `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE` so it never steals
focus. A watchdog relaunches the app when the taskbar window changes (Explorer
restarted).

It's plain Win32 + WinRT through [`windows-rs`](https://github.com/microsoft/windows-rs):
no UI framework, no async runtime, one source file.

### Settings

| Location | Value | Meaning |
| --- | --- | --- |
| `HKCU\Software\WirelessBatteryMonitor` | `Right` | Horizontal position of the overlay (px) |
| `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` | `WirelessBatteryMonitor` | Present when *Start with Windows* is on |

### Known limitations

- A future Windows update could change the taskbar internals and break the
  embedding; the app would then simply show nothing.
- Transparency uses a color key, so text edges can look slightly off on some
  backgrounds or DPI settings.
- Two devices with the same name on the same protocol are both shown (e.g. two
  identical controllers) and are only distinguishable by their position.

## Contributing

Bug reports, device reports, translations and code are all welcome — see
[CONTRIBUTING.md](CONTRIBUTING.md). Look for issues labelled
[`good first issue`](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/labels/good%20first%20issue).
Please follow the [Code of Conduct](CODE_OF_CONDUCT.md); report security problems
privately as described in [SECURITY.md](SECURITY.md).

If the app is useful to you, a ⭐ helps other people find it.

## License

[MIT](LICENSE) © 2026 Pablo González
