#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use windows::core::{w, Interface, HSTRING, IInspectable, PCWSTR, PWSTR};
use windows::Devices::Bluetooth::GenericAttributeProfile::{
    GattCharacteristicUuids, GattCommunicationStatus, GattServiceUuids,
};
use windows::Devices::Bluetooth::{BluetoothCacheMode, BluetoothConnectionStatus, BluetoothDevice, BluetoothLEDevice};
use windows::Devices::Enumeration::Pnp::{PnpObject, PnpObjectType};
use windows::Devices::Enumeration::DeviceInformation;
use windows_collections::IIterable;
use windows::Foundation::{IPropertyValue, PropertyType};
use windows::Storage::Streams::DataReader;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Com::CoIncrementMTAUsage;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture};
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows_registry::CURRENT_USER;

struct Battery {
    name: String,
    level: i32,
    glyph: char,
}

// ---------------------------------------------------------------- Bluetooth

// DEVPKEY_Bluetooth_Battery: set by Windows for classic BT headsets (Hands-Free profile).
const BATTERY_KEY: &str = "{104EA319-6EE2-4701-BD47-8DDBF425BBE5} 2";
const HEADPHONES: char = '\u{E7F6}';
const MOUSE: char = '\u{E962}';
const KEYBOARD: char = '\u{E765}';
const GAMEPAD: char = '\u{E7FC}';
const BLUETOOTH: char = '\u{E702}';

fn read_all() -> Vec<Battery> {
    let mut r = Vec::new();
    let _ = read_classic(&mut r);
    let _ = read_low_energy(&mut r);
    // Classic first, so it wins duplicates (stable sort keeps insertion order for equal names).
    r.sort_by(|a, b| a.name.cmp(&b.name));
    r.dedup_by(|a, b| a.name == b.name);
    r
}

fn props(names: &[&str]) -> IIterable<HSTRING> {
    names.iter().map(|s| HSTRING::from(*s)).collect::<Vec<_>>().into()
}

fn num(v: &IInspectable) -> Option<i64> {
    let p: IPropertyValue = v.cast().ok()?;
    Some(match p.Type().ok()? {
        PropertyType::UInt8 => p.GetUInt8().ok()? as i64,
        PropertyType::Int16 => p.GetInt16().ok()? as i64,
        PropertyType::UInt16 => p.GetUInt16().ok()? as i64,
        PropertyType::Int32 => p.GetInt32().ok()? as i64,
        PropertyType::UInt32 => p.GetUInt32().ok()? as i64,
        _ => return None,
    })
}

fn read_classic(out: &mut Vec<Battery>) -> windows::core::Result<()> {
    const ADDR: &str = "System.Devices.Aep.DeviceAddress";
    let sel = BluetoothDevice::GetDeviceSelectorFromConnectionStatus(BluetoothConnectionStatus::Connected)?;
    let connected = DeviceInformation::FindAllAsyncAqsFilterAndAdditionalProperties(&sel, &props(&[ADDR]))?.join()?;
    if connected.Size()? == 0 {
        return Ok(());
    }
    let key = HSTRING::from(BATTERY_KEY);
    let nodes = PnpObject::FindAllAsync(PnpObjectType::Device, &props(&[BATTERY_KEY]))?.join()?;
    for dev in connected {
        let Ok(mac) = dev.Properties()?.Lookup(&HSTRING::from(ADDR)).and_then(|v| v.cast::<IPropertyValue>()?.GetString())
        else { continue };
        let mac = mac.to_string().replace(':', "").to_uppercase();
        if mac.is_empty() {
            continue;
        }
        let level = (&nodes).into_iter().find_map(|n| {
            if !n.Id().ok()?.to_string().to_uppercase().contains(&mac) {
                return None;
            }
            num(&n.Properties().ok()?.Lookup(&key).ok()?)
        });
        if let Some(level) = level {
            let name = dev.Name()?.to_string();
            // Battery via Hands-Free profile => almost always audio.
            let glyph = glyph_from_name(&name).unwrap_or(HEADPHONES);
            out.push(Battery { name, level: level as i32, glyph });
        }
    }
    Ok(())
}

fn read_low_energy(out: &mut Vec<Battery>) -> windows::core::Result<()> {
    const CAT: &str = "System.Devices.Aep.Bluetooth.Le.Appearance.Category";
    const SUB: &str = "System.Devices.Aep.Bluetooth.Le.Appearance.Subcategory";
    let sel = BluetoothLEDevice::GetDeviceSelectorFromConnectionStatus(BluetoothConnectionStatus::Connected)?;
    let connected = DeviceInformation::FindAllAsyncAqsFilterAndAdditionalProperties(&sel, &props(&[CAT, SUB]))?.join()?;
    for info in connected {
        let Some(level) = read_le_level(&info.Id()?) else { continue };
        let name = info.Name()?.to_string();
        let appearance = || -> Option<char> {
            let p = info.Properties().ok()?;
            if num(&p.Lookup(&HSTRING::from(CAT)).ok()?)? != 0x0F {
                return None; // 0x0F = HID
            }
            match p.Lookup(&HSTRING::from(SUB)).ok().and_then(|v| num(&v)).unwrap_or(0) {
                1 => Some(KEYBOARD),
                2 => Some(MOUSE),
                4 => Some(GAMEPAD),
                _ => None,
            }
        };
        let glyph = glyph_from_name(&name).or_else(appearance).unwrap_or(BLUETOOTH);
        out.push(Battery { name, level, glyph });
    }
    Ok(())
}

fn read_le_level(id: &HSTRING) -> Option<i32> {
    let dev = BluetoothLEDevice::FromIdAsync(id).ok()?.join().ok()?;
    let level = (|| {
        let svcs = dev
            .GetGattServicesForUuidWithCacheModeAsync(GattServiceUuids::Battery().ok()?, BluetoothCacheMode::Uncached)
            .ok()?.join().ok()?;
        if svcs.Status().ok()? != GattCommunicationStatus::Success {
            return None;
        }
        let mut level = None;
        for svc in svcs.Services().ok()? {
            if level.is_none() {
                level = (|| {
                    let chars = svc
                        .GetCharacteristicsForUuidWithCacheModeAsync(
                            GattCharacteristicUuids::BatteryLevel().ok()?, BluetoothCacheMode::Uncached)
                        .ok()?.join().ok()?;
                    if chars.Status().ok()? != GattCommunicationStatus::Success {
                        return None;
                    }
                    let c = chars.Characteristics().ok()?.GetAt(0).ok()?;
                    let read = c.ReadValueWithCacheModeAsync(BluetoothCacheMode::Uncached).ok()?.join().ok()?;
                    if read.Status().ok()? != GattCommunicationStatus::Success {
                        return None;
                    }
                    let buf = read.Value().ok()?;
                    if buf.Length().ok()? == 0 {
                        return None;
                    }
                    Some(DataReader::FromBuffer(&buf).ok()?.ReadByte().ok()? as i32)
                })();
            }
            let _ = svc.Close();
        }
        level
    })();
    let _ = dev.Close(); // release the BLE connection handle
    level
}

fn glyph_from_name(name: &str) -> Option<char> {
    let n = name.to_lowercase();
    let has = |k: &[&str]| k.iter().any(|k| n.contains(k));
    if has(&["mouse", "ratón", "raton", "mx master", "mx anywhere", "pebble"]) { return Some(MOUSE) }
    if has(&["keyboard", "teclado", "keys"]) { return Some(KEYBOARD) }
    if has(&["controller", "xbox", "gamepad", "dualsense", "dualshock", "joy-con"]) { return Some(GAMEPAD) }
    if has(&["bud", "pod", "phone", "wh-", "wf-", "head", "ear", "auricular"]) { return Some(HEADPHONES) }
    None
}

// ---------------------------------------------------------------- UI

const REG_PATH: &str = r"Software\WirelessBatteryMonitor";
const RUN_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_NAME: &str = "WirelessBatteryMonitor";
const LOW_BATTERY: i32 = 20;
const WM_RESULTS: u32 = WM_APP + 1;
const WM_TRAY: u32 = WM_APP + 2;
const TIMER_POLL: usize = 1;
const TIMER_WATCHDOG: usize = 2;
const CMD_REFRESH: usize = 1;
const CMD_STARTUP: usize = 2;
const CMD_EXIT: usize = 3;

struct App {
    host: HWND, // hidden top-level: timers, tray icon, menu owner
    overlay: HWND,
    tip: HWND,
    taskbar: HWND,
    mutex: HANDLE,
    height: i32,
    icon_font: HFONT,
    text_font: HFONT,
    fg: COLORREF,
    bg: COLORREF,
    right: Cell<i32>,
    drag_x: Cell<i32>,
    refreshing: Cell<bool>,
    items: RefCell<Vec<Battery>>,
}

thread_local! { static APP: Cell<Option<&'static App>> = const { Cell::new(None) }; }
fn app() -> Option<&'static App> { APP.with(|a| a.get()) }

const fn rgb(r: u8, g: u8, b: u8) -> COLORREF { COLORREF(r as u32 | (g as u32) << 8 | (b as u32) << 16) }
fn wide(s: &str) -> Vec<u16> { s.encode_utf16().collect() }

fn main() {
    // Keep the MTA alive for the whole process: worker threads join it implicitly, and windows-rs caches
    // WinRT factories that would dangle (crash on next refresh) if the MTA died with a worker thread.
    unsafe { let _ = CoIncrementMTAUsage(); }
    if std::env::args().any(|a| a == "--list") {
        // Debug: `BatteryMonitor.exe --list | more` prints what was detected.
        for b in read_all() {
            println!("{}: {}%", b.name, b.level);
        }
        return;
    }
    unsafe { run() }
}

unsafe fn run() {
    let mutex = CreateMutexW(None, true, w!("WirelessBatteryMonitor")).unwrap_or_default();
    if GetLastError() == ERROR_ALREADY_EXISTS {
        return;
    }
    // ponytail: at logon the taskbar may not exist yet, just wait for it.
    let taskbar = loop {
        if let Ok(h) = FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()) { break h }
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
    let hinst: HINSTANCE = GetModuleHandleW(None).unwrap_or_default().into();
    let _ = InitCommonControlsEx(&INITCOMMONCONTROLSEX {
        dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32,
        dwICC: ICC_WIN95_CLASSES,
    });

    let light = CURRENT_USER
        .open(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
        .and_then(|k| k.get_u32("SystemUsesLightTheme"))
        .is_ok_and(|v| v == 1);
    // ponytail: color-key transparency, text fringes blend toward this color. Per-pixel alpha (UpdateLayeredWindow) if it looks off.
    let bg = if light { rgb(238, 238, 238) } else { rgb(32, 32, 32) };
    let fg = if light { rgb(0, 0, 0) } else { rgb(255, 255, 255) };

    let mut tb = RECT::default();
    let _ = GetWindowRect(taskbar, &mut tb);
    let height = tb.bottom - tb.top;
    // Coordinates are relative to the taskbar (we live inside it as a child window).
    // Default: just left of tray area. Drag to adjust; position is saved.
    let mut tray = RECT::default();
    if let Ok(t) = FindWindowExW(Some(taskbar), None, w!("TrayNotifyWnd"), PCWSTR::null()) {
        let _ = GetWindowRect(t, &mut tray);
    }
    let tray_left = if tray.right > tray.left { tray.left - tb.left } else { tb.right - tb.left - 240 };
    let right = CURRENT_USER.open(REG_PATH).and_then(|k| k.get_u32("Right")).map(|v| v as i32).unwrap_or(tray_left - 4);

    let dpi = GetDpiForWindow(taskbar).max(96) as i32;
    let mut icon_font = font("Segoe Fluent Icons", 110, dpi);
    if !font_exists(icon_font, "Segoe Fluent Icons") {
        let _ = DeleteObject(icon_font.into());
        icon_font = font("Segoe MDL2 Assets", 110, dpi);
    }
    let text_font = font("Segoe UI", 105, dpi);

    let class = |name: PCWSTR| {
        RegisterClassW(&WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: hinst,
            lpszClassName: name,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            ..Default::default()
        });
        name
    };
    let host = CreateWindowExW(
        WINDOW_EX_STYLE::default(), class(w!("BatteryMonitorHost")), w!("BatteryMonitor"), WS_POPUP,
        0, 0, 0, 0, None, None, Some(hinst), None,
    ).unwrap();
    // Win11 taskbar sits above every topmost window, so live inside it as a child window.
    // Created as a layered popup and reparented afterwards: a layered window created directly as a
    // child of another process's window never gets composited (stays invisible).
    let overlay = CreateWindowExW(
        WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_LAYERED, class(w!("BatteryMonitorOverlay")), w!(""),
        WS_POPUP, 0, 0, 1, height, None, None, Some(hinst), None,
    ).unwrap();
    let _ = SetLayeredWindowAttributes(overlay, bg, 0, LWA_COLORKEY);
    let _ = ShowWindow(overlay, SW_SHOWNOACTIVATE);
    SetWindowLongPtrW(overlay, GWL_STYLE, (WS_CHILD | WS_CLIPSIBLINGS | WS_VISIBLE).0 as isize);
    let _ = SetParent(overlay, Some(taskbar));

    let tip = CreateWindowExW(
        WS_EX_TOPMOST, TOOLTIPS_CLASSW, PCWSTR::null(), WINDOW_STYLE(TTS_ALWAYSTIP | TTS_NOPREFIX),
        0, 0, 0, 0, Some(host), None, Some(hinst), None,
    ).unwrap_or_default();
    let mut empty = [0u16];
    let mut ti = tool_info(overlay, PWSTR(empty.as_mut_ptr()));
    SendMessageW(tip, TTM_ADDTOOLW, None, Some(LPARAM(&mut ti as *mut _ as isize)));
    SendMessageW(tip, TTM_SETMAXTIPWIDTH, None, Some(LPARAM(600)));

    // Tray icon: reachable even when overlay is hidden (no devices connected).
    let mut nid = notify_data(host);
    nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    nid.uCallbackMessage = WM_TRAY;
    nid.hIcon = LoadImageW(Some(hinst), PCWSTR(1 as _), IMAGE_ICON,
        GetSystemMetrics(SM_CXSMICON), GetSystemMetrics(SM_CYSMICON), LR_DEFAULTCOLOR)
        .map(|h| HICON(h.0)).unwrap_or_default();
    for (d, s) in nid.szTip.iter_mut().zip(wide("Batería Bluetooth")) { *d = s; }
    let _ = Shell_NotifyIconW(NIM_ADD, &nid);

    let state: &'static App = Box::leak(Box::new(App {
        host, overlay, tip, taskbar, mutex, height, icon_font, text_font, fg, bg,
        right: Cell::new(right), drag_x: Cell::new(0), refreshing: Cell::new(false),
        items: RefCell::new(Vec::new()),
    }));
    APP.with(|a| a.set(Some(state)));

    SetTimer(Some(host), TIMER_POLL, 30_000, None);
    SetTimer(Some(host), TIMER_WATCHDOG, 5_000, None);
    refresh(state);

    let mut msg = MSG::default();
    while GetMessageW(&mut msg, None, 0, 0).as_bool() {
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe fn font(face: &str, tenths_pt: i32, dpi: i32) -> HFONT {
    let mut lf = LOGFONTW { lfHeight: -(tenths_pt * dpi / 720), lfWeight: FW_NORMAL.0 as i32, ..Default::default() };
    for (d, s) in lf.lfFaceName.iter_mut().zip(wide(face)) { *d = s; }
    CreateFontIndirectW(&lf)
}

unsafe fn font_exists(f: HFONT, face: &str) -> bool {
    let dc = GetDC(None);
    let old = SelectObject(dc, f.into());
    let mut buf = [0u16; 64];
    let n = GetTextFaceW(dc, Some(&mut buf));
    SelectObject(dc, old);
    ReleaseDC(None, dc);
    n > 0 && String::from_utf16_lossy(&buf[..(n as usize).saturating_sub(1)]).eq_ignore_ascii_case(face)
}

fn tool_info(overlay: HWND, text: PWSTR) -> TTTOOLINFOW {
    TTTOOLINFOW {
        cbSize: size_of::<TTTOOLINFOW>() as u32,
        uFlags: TTF_IDISHWND | TTF_SUBCLASS,
        hwnd: overlay,
        uId: overlay.0 as usize,
        lpszText: text,
        ..Default::default()
    }
}

fn notify_data(host: HWND) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW { cbSize: size_of::<NOTIFYICONDATAW>() as u32, hWnd: host, uID: 1, ..Default::default() }
}

fn refresh(a: &App) {
    if a.refreshing.replace(true) {
        return;
    }
    let host = a.host.0 as isize; // HWND isn't Send
    std::thread::spawn(move || unsafe {
        let items = Box::into_raw(Box::new(read_all()));
        if PostMessageW(Some(HWND(host as _)), WM_RESULTS, WPARAM(0), LPARAM(items as isize)).is_err() {
            drop(Box::from_raw(items));
        }
    });
}

/// Measures (and optionally draws) the items; returns total width.
unsafe fn layout(a: &App, dc: HDC, draw: bool) -> i32 {
    SetBkMode(dc, TRANSPARENT);
    let old = SelectObject(dc, a.text_font.into());
    let mut x = 6;
    for b in a.items.borrow().iter() {
        let color = if b.level <= LOW_BATTERY { rgb(255, 99, 71) } else { a.fg };
        SetTextColor(dc, color);
        for (f, s, gap) in [(a.icon_font, wide(&b.glyph.to_string()), 3), (a.text_font, wide(&format!("{}%", b.level)), 12)] {
            SelectObject(dc, f.into());
            let mut sz = SIZE::default();
            let _ = GetTextExtentPoint32W(dc, &s, &mut sz);
            if draw {
                let _ = TextOutW(dc, x, (a.height - sz.cy) / 2, &s);
            }
            x += sz.cx + gap;
        }
    }
    SelectObject(dc, old);
    x - 12 + 6
}

unsafe fn relayout(a: &App) {
    let n = a.items.borrow().len();
    let dc = GetDC(Some(a.overlay));
    let w = layout(a, dc, false);
    ReleaseDC(Some(a.overlay), dc);
    let _ = SetWindowPos(a.overlay, None, a.right.get() - w, 0, w, a.height, SWP_NOZORDER | SWP_NOACTIVATE);
    let text = a.items.borrow().iter().map(|b| format!("{}: {}%", b.name, b.level)).collect::<Vec<_>>().join("\n");
    let mut buf: Vec<u16> = wide(&text).into_iter().chain([0]).collect();
    let ti = tool_info(a.overlay, PWSTR(buf.as_mut_ptr()));
    SendMessageW(a.tip, TTM_UPDATETIPTEXTW, None, Some(LPARAM(&ti as *const _ as isize)));
    let _ = ShowWindow(a.overlay, if n > 0 { SW_SHOWNOACTIVATE } else { SW_HIDE });
    let _ = InvalidateRect(Some(a.overlay), None, true);
}

/// Overlay rect in taskbar coordinates.
unsafe fn overlay_rect(a: &App) -> RECT {
    let mut r = RECT::default();
    let _ = GetWindowRect(a.overlay, &mut r);
    let mut pts = [POINT { x: r.left, y: r.top }, POINT { x: r.right, y: r.bottom }];
    MapWindowPoints(None, Some(a.taskbar), &mut pts);
    RECT { left: pts[0].x, top: pts[0].y, right: pts[1].x, bottom: pts[1].y }
}

fn run_at_startup() -> bool {
    CURRENT_USER.open(RUN_PATH).and_then(|k| k.get_string(APP_NAME)).is_ok()
}

fn set_run_at_startup(on: bool) {
    let Ok(key) = CURRENT_USER.create(RUN_PATH) else { return };
    let exe = std::env::current_exe().unwrap_or_default();
    let _ = if on { key.set_string(APP_NAME, format!("\"{}\"", exe.display())) } else { key.remove_value(APP_NAME) };
}

unsafe fn show_menu(a: &App) {
    let Ok(menu) = CreatePopupMenu() else { return };
    let startup = run_at_startup();
    let _ = AppendMenuW(menu, MF_STRING, CMD_REFRESH, w!("Actualizar"));
    let _ = AppendMenuW(menu, MF_STRING | if startup { MF_CHECKED } else { MF_UNCHECKED }, CMD_STARTUP, w!("Iniciar con Windows"));
    let _ = AppendMenuW(menu, MF_STRING, CMD_EXIT, w!("Salir"));
    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    // Foreground + WM_NULL: standard dance so the menu closes when clicking elsewhere.
    let _ = SetForegroundWindow(a.host);
    let cmd = TrackPopupMenu(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON | TPM_BOTTOMALIGN, pt.x, pt.y, None, a.host, None);
    let _ = PostMessageW(Some(a.host), WM_NULL, WPARAM(0), LPARAM(0));
    let _ = DestroyMenu(menu);
    match cmd.0 as usize {
        CMD_REFRESH => refresh(a),
        CMD_STARTUP => set_run_at_startup(!startup),
        CMD_EXIT => quit(a),
        _ => {}
    }
}

unsafe fn quit(a: &App) {
    let _ = Shell_NotifyIconW(NIM_DELETE, &notify_data(a.host)); // else ghost icon lingers in tray
    let _ = DestroyWindow(a.overlay); // detach from explorer's taskbar
    PostQuitMessage(0);
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let Some(a) = app() else { return DefWindowProcW(hwnd, msg, wp, lp) };
    let x = (lp.0 & 0xFFFF) as i16 as i32;
    match msg {
        WM_TIMER if wp.0 == TIMER_POLL => refresh(a),
        WM_TIMER if wp.0 == TIMER_WATCHDOG => {
            // ponytail: explorer.exe restart destroys the taskbar (and us with it) -> just restart the app.
            if let Ok(tb) = FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()) {
                if tb != a.taskbar {
                    let _ = CloseHandle(a.mutex);
                    if let Ok(exe) = std::env::current_exe() {
                        let _ = std::process::Command::new(exe).spawn();
                    }
                    quit(a);
                }
            }
        }
        WM_RESULTS => {
            *a.items.borrow_mut() = *Box::from_raw(lp.0 as *mut Vec<Battery>);
            a.refreshing.set(false);
            relayout(a);
        }
        WM_TRAY if matches!((lp.0 & 0xFFFF) as u32, WM_RBUTTONUP | WM_LBUTTONUP) => show_menu(a),
        WM_PAINT if hwnd == a.overlay => {
            let mut ps = PAINTSTRUCT::default();
            let dc = BeginPaint(hwnd, &mut ps);
            let brush = CreateSolidBrush(a.bg);
            FillRect(dc, &ps.rcPaint, brush);
            let _ = DeleteObject(brush.into());
            layout(a, dc, true);
            let _ = EndPaint(hwnd, &ps);
        }
        WM_MOUSEACTIVATE => return LRESULT(MA_NOACTIVATE as isize),
        // Horizontal drag only; saves position on release.
        WM_LBUTTONDOWN => {
            a.drag_x.set(x);
            SetCapture(hwnd);
        }
        WM_MOUSEMOVE if wp.0 & 0x0001 != 0 => {
            let r = overlay_rect(a);
            let _ = SetWindowPos(hwnd, None, r.left + x - a.drag_x.get(), r.top, 0, 0,
                SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
        }
        WM_LBUTTONUP => {
            let _ = ReleaseCapture();
            a.right.set(overlay_rect(a).right);
            if let Ok(k) = CURRENT_USER.create(REG_PATH) {
                let _ = k.set_u32("Right", a.right.get() as u32);
            }
        }
        // Show menu manually: WM_CONTEXTMENU doesn't arrive reliably inside explorer's taskbar.
        WM_RBUTTONUP if hwnd == a.overlay => show_menu(a),
        _ => return DefWindowProcW(hwnd, msg, wp, lp),
    }
    LRESULT(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn glyphs() {
        assert_eq!(glyph_from_name("MX Master 3S"), Some(MOUSE));
        assert_eq!(glyph_from_name("Ratón Logi"), Some(MOUSE));
        assert_eq!(glyph_from_name("Xbox Wireless Controller"), Some(GAMEPAD));
        assert_eq!(glyph_from_name("WH-1000XM4"), Some(HEADPHONES));
        assert_eq!(glyph_from_name("K380 Keyboard"), Some(KEYBOARD));
        assert_eq!(glyph_from_name("Thing"), None);
    }
}
