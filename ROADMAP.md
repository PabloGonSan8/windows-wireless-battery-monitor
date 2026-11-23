# Roadmap

What's planned, roughly in priority order. Nothing here is promised for a date.
Want something? Comment on (or 👍) the linked issue, or open a
[feature request](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=feature_request.yml).

Guiding principle: stay a tiny, single-purpose, dependency-light app. Features that
need a settings window, a service or an installer must clearly earn their place.

## Next (1.2)

- [ ] **Configurable low-battery threshold** (today fixed at 20 %), from the menu.
- [ ] **Hide a device** from the overlay (e.g. a headset you don't care about), from the menu.
- [ ] **Turn low-battery notifications on/off** from the menu.
- [ ] **Verify Windows 10** support and document the result.
- [ ] **Taskbar on top / on the side / multi-monitor**: test and fix the positioning.

## Distribution

- [ ] **Code signing** to remove the SmartScreen warning (e.g. SignPath Foundation for OSS).
- [ ] **winget** package (`winget install …`).
- [ ] **Scoop** manifest.
- [ ] ARM64 build (Windows on Snapdragon), if there are users asking for it.

## Later / ideas under discussion

- Per-pixel alpha (`UpdateLayeredWindow`) instead of the color key, for smoother text edges.
- Better device-type detection (more name patterns, LE appearance values).
- Show the battery of each earbud / the case, where Windows exposes it.
- More UI languages (the menu has only five strings).

## Not planned

- Vendor-specific protocols (Logitech HID++, Razer, Apple AirPods…): they require
  reverse-engineered, per-brand code and break often. Contributions that keep it
  isolated and optional could be discussed in an issue first.
- A full settings window or a background service.

## Done

- 1.1.0 — English/Spanish UI, low-battery notification, live light/dark switch,
  identical devices no longer merged, `--list` prints to the console, automated releases.
- 1.0.0 — First release.
