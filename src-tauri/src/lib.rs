// 桌面小 Claude：主程式
// 視窗做法沿用 quietpet-tauri：透明、無邊框、蓋滿主螢幕工作區，
// 每 40ms 讀一次滑鼠位置，只有在小 Claude 或對話泡泡上才可以點，其他地方點下去會穿透到後面。

use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

/// 畫面回報的「可以點的區域」（CSS px，相對於視窗左上角）
#[derive(Default)]
struct Hit {
    rects: Vec<[f64; 4]>,
    capture: bool, // 正在拖曳時一律不穿透
}

struct Shared {
    hit: Arc<Mutex<Hit>>,
}

#[tauri::command]
fn set_hit(state: tauri::State<Shared>, rects: Vec<[f64; 4]>, capture: bool) {
    if let Ok(mut h) = state.hit.lock() {
        h.rects = rects;
        h.capture = capture;
    }
}

#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) {
    let m = app.autolaunch();
    let _ = if on { m.enable() } else { m.disable() };
}

#[tauri::command]
fn show_menu(app: AppHandle, window: WebviewWindow, class_mode: bool, size: String) -> Result<(), String> {
    let e = |e: tauri::Error| e.to_string();
    let auto = app.autolaunch().is_enabled().unwrap_or(false);
    let class = CheckMenuItem::with_id(&app, "class", "上課模式（不出聲）", true, class_mode, None::<&str>).map_err(e)?;
    let s = CheckMenuItem::with_id(&app, "size:s", "小", true, size == "s", None::<&str>).map_err(e)?;
    let m = CheckMenuItem::with_id(&app, "size:m", "中", true, size == "m", None::<&str>).map_err(e)?;
    let l = CheckMenuItem::with_id(&app, "size:l", "大", true, size == "l", None::<&str>).map_err(e)?;
    let sizes = Submenu::with_items(&app, "大小", true, &[&s, &m, &l]).map_err(e)?;
    let check = MenuItem::with_id(&app, "check", "立即檢查消息", true, None::<&str>).map_err(e)?;
    let hide = MenuItem::with_id(&app, "hide10", "暫時隱藏 10 分鐘", true, None::<&str>).map_err(e)?;
    let auto_item = CheckMenuItem::with_id(&app, "autostart", "開機自動啟動", true, auto, None::<&str>).map_err(e)?;
    let quit = MenuItem::with_id(&app, "quit", "關閉小 Claude", true, None::<&str>).map_err(e)?;
    let sep1 = PredefinedMenuItem::separator(&app).map_err(e)?;
    let sep2 = PredefinedMenuItem::separator(&app).map_err(e)?;
    let menu = Menu::with_items(&app, &[&class, &sizes, &sep1, &check, &hide, &auto_item, &sep2, &quit]).map_err(e)?;
    window.popup_menu(&menu).map_err(e)
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_always_on_top(true);
    }
}

fn toggle_visible(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        if w.is_visible().unwrap_or(true) {
            let _ = w.hide();
        } else {
            show_main(app);
        }
    }
}

/// 加上 WS_EX_NOACTIVATE：點小 Claude 時不會把 PowerPoint 的焦點搶走（翻頁鍵仍給簡報）
#[cfg(windows)]
fn ensure_no_activate(win: &WebviewWindow) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_NOACTIVATE};
    if let Ok(h) = win.hwnd() {
        let h = h.0 as windows_sys::Win32::Foundation::HWND;
        unsafe {
            let ex = GetWindowLongPtrW(h, GWL_EXSTYLE);
            let want = ex | WS_EX_NOACTIVATE as isize;
            if want != ex {
                SetWindowLongPtrW(h, GWL_EXSTYLE, want);
            }
        }
    }
}
#[cfg(not(windows))]
fn ensure_no_activate(_win: &WebviewWindow) {}

/// 視窗蓋滿主螢幕的工作區（不含工作列）
fn fit_to_screen(win: &WebviewWindow) {
    if let Ok(Some(m)) = win.primary_monitor() {
        let wa = m.work_area();
        let _ = win.set_position(PhysicalPosition::new(wa.position.x, wa.position.y));
        let _ = win.set_size(PhysicalSize::new(wa.size.width, wa.size.height));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let hit = Arc::new(Mutex::new(Hit::default()));

    tauri::Builder::default()
        // 已經開著時再雙擊 exe，只把原本的小 Claude 叫出來，不會出現兩隻
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_main(app)))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .manage(Shared { hit: hit.clone() })
        .invoke_handler(tauri::generate_handler![set_hit, set_autostart, show_menu])
        .on_menu_event(|app, event| {
            let id = event.id().as_ref().to_string();
            match id.as_str() {
                "class" => {
                    let _ = app.emit("command", "class:toggle");
                }
                "check" => {
                    let _ = app.emit("command", "check");
                }
                "hide10" => {
                    if let Some(w) = app.get_webview_window("main") {
                        let _ = w.hide();
                    }
                    let a = app.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_secs(600));
                        show_main(&a);
                    });
                }
                "autostart" => {
                    let m = app.autolaunch();
                    let on = m.is_enabled().unwrap_or(false);
                    let _ = if on { m.disable() } else { m.enable() };
                }
                "toggle_vis" => toggle_visible(app),
                "quit" => app.exit(0),
                s if s.starts_with("size:") => {
                    let _ = app.emit("command", s);
                }
                _ => {}
            }
        })
        .setup(move |app| {
            // 系統匣（右下角時鐘旁）
            let show = MenuItem::with_id(app, "toggle_vis", "顯示／隱藏小 Claude", true, None::<&str>)?;
            let check = MenuItem::with_id(app, "check", "立即檢查消息", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "關閉小 Claude", true, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let tray_menu = Menu::with_items(app, &[&show, &check, &sep, &quit])?;
            if let Some(icon) = app.default_window_icon() {
                TrayIconBuilder::new()
                    .icon(icon.clone())
                    .tooltip("小 Claude")
                    .menu(&tray_menu)
                    .show_menu_on_left_click(true)
                    .build(app)?;
            }

            let win = app.get_webview_window("main").expect("main window");
            fit_to_screen(&win);
            let _ = win.set_ignore_cursor_events(true);
            ensure_no_activate(&win);
            let _ = win.show();
            let _ = win.set_always_on_top(true);

            // 滑鼠輪詢：決定穿透與否
            let w = win.clone();
            let hit = hit.clone();
            std::thread::spawn(move || {
                let mut ignoring: Option<bool> = Some(true);
                let mut tick: u64 = 0;
                loop {
                    std::thread::sleep(Duration::from_millis(40));
                    tick += 1;
                    let (Ok(c), Ok(p), Ok(s)) = (w.cursor_position(), w.outer_position(), w.scale_factor()) else {
                        continue;
                    };
                    let x = (c.x - p.x as f64) / s;
                    let y = (c.y - p.y as f64) / s;
                    let inside = match hit.lock() {
                        Ok(h) => h.capture || h.rects.iter().any(|r| x >= r[0] && x <= r[0] + r[2] && y >= r[1] && y <= r[1] + r[3]),
                        Err(_) => false,
                    };
                    if ignoring != Some(!inside) {
                        let _ = w.set_ignore_cursor_events(!inside);
                        ignoring = Some(!inside);
                    }
                    ensure_no_activate(&w);
                    // 每 4 秒重新置頂（簡報放映時可能被壓下去）
                    if tick % 100 == 0 && w.is_visible().unwrap_or(false) {
                        let _ = w.set_always_on_top(true);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
