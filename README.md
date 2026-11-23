# Wireless Battery Monitor

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue)

A tiny Windows taskbar overlay that shows the battery level of your connected
Bluetooth devices (mouse, keyboard, headphones, controllers) right next to the
system tray. No installer, no background service, no runtime: a single
~300 KB native executable written in Rust (~3 MB of private memory).

![Wireless Battery Monitor overlay in the Windows 11 taskbar](docs/screenshot.png)

> Screenshot with example values: mouse 87 %, keyboard 64 %, headphones 41 %
> and a gamepad at 14 % (red because it is below 20 %). Your taskbar will show
> your own devices only.

## Features

- Battery percentage for **Bluetooth Classic** (Hands-Free profile) and
  **Bluetooth LE** (GATT Battery Service) devices.
- Device-type icons (mouse, keyboard, headset, gamepad) using
  *Segoe Fluent Icons* / *Segoe MDL2 Assets*.
- Red highlight when a device drops to **20 % or below**.
- Drag to reposition; the position is remembered across restarts.
- Tray icon with a menu: refresh, start with Windows, exit.
- Survives `explorer.exe` restarts (the overlay relaunches itself).
- Single instance: launching it twice does nothing.

## Requirements

- **Windows 10** (2004+) or **Windows 11**.
- [Rust](https://rustup.rs) (MSVC toolchain) to build.
- Devices must expose their battery level through the Hands-Free profile
  (classic) or the standard GATT Battery Service (LE). Many devices do not —
  see [Limitations](#limitations).

## Build and run

```powershell
cargo run --release
```

The executable is `target/release/BatteryMonitor.exe`; copy it anywhere, it has
no dependencies.

## Usage

- Launch the app. The overlay appears in the taskbar, just left of the tray.
- **Left-drag** the overlay to move it horizontally.
- **Right-click** it (or the tray icon) to open the menu:
  - *Refresh* — re-scan the connected devices.
  - *Start with Windows* — adds/removes a `HKCU\...\Run` entry.
  - *Exit*.
- Hover the overlay to see the full device list in a tooltip.

### Debug mode

```powershell
BatteryMonitor.exe --list
```

Prints the detected devices and their levels to the console, without starting
the overlay. Useful to check why a device is not showing up.

### Settings

The app stores one value under `HKCU\Software\WirelessBatteryMonitor`:

| Value   | Meaning                                    |
| ------- | ------------------------------------------ |
| `Right` | Horizontal position of the overlay (px).   |

The autostart entry lives under `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
as `WirelessBatteryMonitor`.

## How it works

1. Enumerates connected Bluetooth Classic devices and matches them with PnP
   nodes that publish `DEVPKEY_Bluetooth_Battery`.
2. Enumerates connected Bluetooth LE devices, opens the GATT **Battery
   Service** (`0x180F`) and reads the **Battery Level** characteristic (`0x2A19`).
3. Repeats every 30 seconds and redraws the overlay.

To sit on the Windows 11 taskbar, the window is reparented as a child of
`Shell_TrayWnd` using `SetParent`, with `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE`
so it never steals focus. A watchdog restarts the app when the taskbar window
changes (i.e. Explorer restarted).

## Limitations

- Windows-only, and the taskbar embedding is only tested on Windows 11.
  Windows 10 may work but is untested.
- Devices that do not expose a battery level to Windows will not appear.
  AirPods and some Sony/Logitech models are known examples.
- A future Windows update could change the taskbar internals and break the
  embedding. The app degrades gracefully: it just won't show anything.
- Colors are color-keyed for transparency, so text fringes can look slightly
  off on some backgrounds/DPI settings.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md)
before opening a pull request. For security issues, please do not open a
public issue — contact the maintainer directly.

## License

[MIT](LICENSE) © 2026 Pablo González

---

# Wireless Battery Monitor (Español)

Pequeño overlay para la barra de tareas de Windows que muestra el nivel de
batería de los dispositivos Bluetooth conectados (ratón, teclado, auriculares,
mandos) junto a la bandeja del sistema. Sin instalador, sin servicio en
segundo plano y sin runtime: un único ejecutable nativo de ~300 KB escrito en
Rust (~3 MB de memoria privada).

![Overlay de Wireless Battery Monitor en la barra de tareas de Windows 11](docs/screenshot.png)

> Captura con datos de ejemplo: ratón 87 %, teclado 64 %, auriculares 41 % y
> un mando al 14 % (en rojo porque está por debajo del 20 %). En tu equipo
> aparecerán solo tus dispositivos.

## Características

- Porcentaje de batería de dispositivos **Bluetooth Classic** (perfil
  Hands-Free) y **Bluetooth LE** (servicio GATT de batería).
- Iconos según el tipo de dispositivo (ratón, teclado, auriculares, mando)
  usando *Segoe Fluent Icons* / *Segoe MDL2 Assets*.
- Se resalta en rojo cuando un dispositivo baja al **20 % o menos**.
- Se puede arrastrar para recolocarlo; la posición se guarda entre reinicios.
- Icono en la bandeja con menú: actualizar, iniciar con Windows, salir.
- Sobrevive a reinicios de `explorer.exe` (se relanza solo).
- Instancia única: si lo abres dos veces, no pasa nada.

## Requisitos

- **Windows 10** (2004 o superior) o **Windows 11**.
- [Rust](https://rustup.rs) (toolchain MSVC) para compilar.
- Los dispositivos deben exponer su batería mediante el perfil Hands-Free
  (classic) o el servicio GATT de batería (LE). Muchos no lo hacen; consulta
  [Limitaciones](#limitaciones-1).

## Compilar y ejecutar

```powershell
cargo run --release
```

El ejecutable queda en `target/release/BatteryMonitor.exe`; puedes copiarlo a
cualquier sitio, no tiene dependencias.

## Uso

- Al abrirlo, el overlay aparece en la barra de tareas, a la izquierda de la
  bandeja del sistema.
- **Arrastra con el botón izquierdo** para moverlo horizontalmente.
- **Clic derecho** sobre el overlay (o el icono de la bandeja) para abrir el
  menú: *Actualizar*, *Iniciar con Windows* y *Salir*.
- Pasa el ratón por encima para ver la lista completa de dispositivos en un
  tooltip.

### Modo depuración

```powershell
BatteryMonitor.exe --list
```

Imprime en consola los dispositivos detectados y su nivel, sin abrir el
overlay. Útil para averiguar por qué un dispositivo no aparece.

### Configuración

La aplicación guarda sus datos en `HKCU\Software\WirelessBatteryMonitor`
(valor `Right`: posición horizontal del overlay). La entrada de inicio
automático está en `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` como
`WirelessBatteryMonitor`.

## Cómo funciona

1. Enumera los dispositivos Bluetooth Classic conectados y los cruza con los
   nodos PnP que publican `DEVPKEY_Bluetooth_Battery`.
2. Enumera los dispositivos Bluetooth LE conectados, abre el servicio GATT de
   batería (`0x180F`) y lee la característica de nivel (`0x2A19`).
3. Repite el proceso cada 30 segundos y redibuja el overlay.

Para colocarse sobre la barra de tareas de Windows 11, la ventana se convierte
en hija de `Shell_TrayWnd` con `SetParent`, usando
`WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE` para no robar nunca el foco. Un watchdog
reinicia la aplicación cuando la ventana de la barra de tareas cambia (es
decir, cuando Explorer se reinicia).

## Limitaciones

- Solo Windows, y la integración con la barra de tareas solo está probada en
  Windows 11. En Windows 10 puede funcionar, pero no está verificado.
- Los dispositivos que no exponen la batería a Windows no aparecerán (por
  ejemplo, AirPods y algunos modelos de Sony y Logitech).
- Una futura actualización de Windows podría cambiar el funcionamiento interno
  de la barra de tareas y romper la integración. La aplicación simplemente
  dejaría de mostrar nada.
- Los colores usan *color key* para la transparencia, así que los bordes del
  texto pueden verse ligeramente raros con ciertos fondos y escalados de DPI.

## Contribuir

Las contribuciones son bienvenidas. Lee [CONTRIBUTING.md](CONTRIBUTING.md)
antes de abrir un pull request. Para problemas de seguridad, no abras un issue
público: contacta directamente con el mantenedor.

## Licencia

[MIT](LICENSE) © 2026 Pablo González
