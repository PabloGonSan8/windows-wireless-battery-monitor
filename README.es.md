<div align="center">

# Wireless Battery Monitor

**La batería de tu ratón, teclado, auriculares y mandos Bluetooth, siempre visible
en la barra de tareas de Windows.**

[![Última versión](https://img.shields.io/github/v/release/PabloGonSan8/windows-wireless-battery-monitor?label=descargar&color=2ea44f)](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest)
[![Build](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/actions/workflows/build.yml/badge.svg)](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/actions/workflows/build.yml)
![Windows 10 / 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4)
[![Licencia: MIT](https://img.shields.io/badge/licencia-MIT-yellow.svg)](LICENSE)

![Wireless Battery Monitor en la barra de tareas de Windows 11](docs/screenshot.png)

<sub>Valores de ejemplo: ratón 87 %, teclado 64 %, auriculares 41 % y un mando al 14 % (en rojo: por debajo del 20 %).</sub>

### [⬇ Descargar para Windows](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest)

Un único `.exe` de ~300 KB · sin instalador · sin servicios en segundo plano · sin telemetría

[English](README.md)

</div>

## ¿Por qué?

Windows ya conoce la batería de muchos dispositivos Bluetooth, pero la esconde en
*Configuración → Bluetooth y dispositivos*. Te enteras de que el ratón está sin
batería cuando deja de moverse.

Wireless Battery Monitor muestra la batería de todos los dispositivos conectados
directamente en la barra de tareas, junto al reloj: de un vistazo, sin clics y
sin una aplicación distinta para cada marca.

## Características

- 🔋 **Todos tus dispositivos a la vez**, dentro de la barra de tareas (no escondidos tras un icono).
- 🎧 **Bluetooth Classic** (perfil Hands-Free) y **Bluetooth Low Energy** (servicio GATT de batería).
- 🖱️ **Iconos** de ratón, teclado, auriculares y mando.
- 🔴 **Aviso de batería baja**: el nivel se pone en rojo al 20 % o menos y Windows
  muestra una notificación una vez cuando el dispositivo llega a ese nivel.
- 🌗 Sigue el **tema claro/oscuro** de Windows, también si lo cambias en caliente.
- ↔️ **Arrastrable**; la posición se recuerda.
- 🚀 Opción **Iniciar con Windows** en el menú.
- 🌍 Menú en **español o inglés** según el idioma de Windows.
- 🪶 **Ligero**: un ejecutable nativo en Rust de ~300 KB y ~5 MB de memoria privada
  (medido en Windows 11). Sobrevive a reinicios de `explorer.exe`.

## Empezar

1. Descarga `WirelessBatteryMonitor.exe` (o el `.zip` portable) de la
   [última versión](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/releases/latest).
2. Guárdalo donde quieras (por ejemplo `C:\Herramientas\`) y haz doble clic.
3. Tus dispositivos aparecen en la barra de tareas, a la izquierda de los iconos de la bandeja.
   Clic derecho → **Iniciar con Windows** para que arranque solo.

> [!NOTE]
> **¿"Windows protegió su PC"?** El ejecutable aún no está firmado (los
> certificados cuestan dinero; está en el [roadmap](ROADMAP.md)). Pulsa
> **Más información → Ejecutar de todas formas**. Cada versión la compila
> [GitHub Actions](.github/workflows/release.yml) a partir del código público e
> incluye `SHA256SUMS.txt` y una atestación de origen para verificarla.

| Acción | Resultado |
| --- | --- |
| Pasar el ratón por encima | Tooltip con nombres completos y niveles |
| Arrastrar con el botón izquierdo | Moverlo en horizontal (se guarda la posición) |
| Clic derecho en el overlay o en el icono de la bandeja | Menú: **Actualizar**, **Iniciar con Windows**, **Salir** |

Los niveles se actualizan cada 30 segundos.

## ¿Funciona con mi dispositivo?

**Regla general:** si Windows muestra el porcentaje del dispositivo en
*Configuración → Bluetooth y dispositivos*, normalmente esta aplicación también lo mostrará.
Lee las mismas fuentes estándar:

- Dispositivos Classic que informan de la batería por el **perfil Hands-Free** (la mayoría de auriculares).
- Dispositivos LE con el **servicio GATT de batería** estándar (muchos ratones, teclados y mandos).

Los dispositivos que solo informan a través de la aplicación del fabricante o de un
receptor USB (por ejemplo, Logitech Unifying/Bolt) no son visibles para Windows y no
aparecerán. Los AirPods son un ejemplo conocido de dispositivo que no expone su batería a Windows.

📋 Consulta la **[lista de compatibilidad](COMPATIBILITY.md)** y, por favor,
**[informa de tu dispositivo](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml)**,
funcione o no. No hace falta saber programar; son dos minutos.

## Preguntas frecuentes

<details>
<summary><b>Mi dispositivo está conectado pero no aparece.</b></summary>

Abre una terminal en la carpeta del ejecutable y ejecuta:

```powershell
.\WirelessBatteryMonitor.exe --list | more
```

Muestra todos los dispositivos y niveles que la aplicación puede leer. Si el tuyo no
sale, es que Windows no está recibiendo su nivel de batería (compruébalo en
*Configuración → Bluetooth y dispositivos*). [Infórmanos](https://github.com/PabloGonSan8/windows-wireless-battery-monitor/issues/new?template=device_report.yml)
para añadirlo a la lista de compatibilidad.
</details>

<details>
<summary><b>¿Es seguro? El antivirus o SmartScreen me avisa.</b></summary>

La aplicación no está firmada, así que SmartScreen avisa hasta que gane reputación.
**No se conecta a internet**, no recoge datos y todo el código está en un único
archivo: [`src/main.rs`](src/main.rs). Puedes verificar la descarga con:

```powershell
Get-FileHash .\WirelessBatteryMonitor.exe -Algorithm SHA256   # compara con SHA256SUMS.txt
gh attestation verify .\WirelessBatteryMonitor.exe -R PabloGonSan8/windows-wireless-battery-monitor
```
</details>

<details>
<summary><b>¿Funciona en Windows 10? ¿Con la barra de tareas arriba o en un lateral?</b></summary>

Se desarrolla y prueba en Windows 11 con la barra abajo. Windows 10 (2004+) y otras
posiciones **no están probadas**; los informes son muy bienvenidos.
</details>

<details>
<summary><b>¿Cómo lo desinstalo?</b></summary>

1. Clic derecho en el overlay y desmarca **Iniciar con Windows**.
2. Pulsa **Salir** y borra el `.exe`.
3. Opcional: borra la clave `HKCU\Software\WirelessBatteryMonitor` (solo guarda la posición).
</details>

<details>
<summary><b>¿Puedo cambiar el umbral del 20 % u ocultar un dispositivo?</b></summary>

Todavía no. Ambas cosas están en el [roadmap](ROADMAP.md).
</details>

## Para desarrolladores

Necesitas [Rust](https://rustup.rs) con la toolchain MSVC.

```powershell
cargo run --release              # arranca el overlay
cargo run --release -- --list    # muestra los dispositivos detectados y sale
cargo test
```

El funcionamiento interno, la configuración en el registro y las limitaciones
conocidas están documentados en el [README en inglés](README.md#for-developers).

## Contribuir

Informes de errores, de dispositivos, traducciones y código: todo suma. Lee
[CONTRIBUTING.md](CONTRIBUTING.md) y el [Código de conducta](CODE_OF_CONDUCT.md).
Los problemas de seguridad se informan en privado según [SECURITY.md](SECURITY.md).

Si te resulta útil, una ⭐ ayuda a que otras personas lo encuentren.

## Licencia

[MIT](LICENSE) © 2026 Pablo González
