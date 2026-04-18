#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State, RunEvent, WindowEvent};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_notification::NotificationExt;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimerMode {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Serialize)]
pub struct TimerStatePayload {
    pub seconds_remaining: u32,
    pub mode: TimerMode,
    pub is_running: bool,
}

pub struct AppState {
    pub seconds_remaining: u32,
    pub mode: TimerMode,
    pub is_running: bool,
    pub work_duration: u32,
    pub short_break_duration: u32,
    pub long_break_duration: u32,
    pub pomodoros_completed: u32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            seconds_remaining: 25 * 60,
            mode: TimerMode::Work,
            is_running: false,
            work_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            pomodoros_completed: 0,
        }
    }
}

pub struct AppStateMutex(pub Mutex<AppState>);

#[tauri::command]
fn start_timer(state: State<AppStateMutex>, app: AppHandle) {
    let mut state = state.0.lock().unwrap();
    state.is_running = true;
    emit_state(&state, &app);
}

#[tauri::command]
fn pause_timer(state: State<AppStateMutex>, app: AppHandle) {
    let mut state = state.0.lock().unwrap();
    state.is_running = false;
    emit_state(&state, &app);
}

#[tauri::command]
fn reset_timer(state: State<AppStateMutex>, app: AppHandle) {
    let mut state = state.0.lock().unwrap();
    state.is_running = false;
    state.seconds_remaining = match state.mode {
        TimerMode::Work => state.work_duration * 60,
        TimerMode::ShortBreak => state.short_break_duration * 60,
        TimerMode::LongBreak => state.long_break_duration * 60,
    };
    emit_state(&state, &app);
}

#[tauri::command]
fn set_durations(
    work: u32,
    short_break: u32,
    long_break: u32,
    state: State<AppStateMutex>,
    app: AppHandle,
) {
    let mut state = state.0.lock().unwrap();
    state.work_duration = work;
    state.short_break_duration = short_break;
    state.long_break_duration = long_break;

    if !state.is_running {
        state.seconds_remaining = match state.mode {
            TimerMode::Work => state.work_duration * 60,
            TimerMode::ShortBreak => state.short_break_duration * 60,
            TimerMode::LongBreak => state.long_break_duration * 60,
        };
    }
    emit_state(&state, &app);
}

#[tauri::command]
fn set_mode(mode: TimerMode, state: State<AppStateMutex>, app: AppHandle) {
    let mut state = state.0.lock().unwrap();
    state.mode = mode.clone();
    state.is_running = false;
    state.seconds_remaining = match mode {
        TimerMode::Work => state.work_duration * 60,
        TimerMode::ShortBreak => state.short_break_duration * 60,
        TimerMode::LongBreak => state.long_break_duration * 60,
    };
    emit_state(&state, &app);
}

fn emit_state(state: &AppState, app: &AppHandle) {
    let payload = TimerStatePayload {
        seconds_remaining: state.seconds_remaining,
        mode: state.mode.clone(),
        is_running: state.is_running,
    };
    let _ = app.emit("timer-tick", payload);
}

pub fn start_background_thread(app: AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));

        let state_mutex = app.state::<AppStateMutex>();
        let mut state = state_mutex.0.lock().unwrap();

        if state.is_running {
            if state.seconds_remaining > 0 {
                state.seconds_remaining -= 1;
            } else {
                state.is_running = false;

                let title;
                let body;

                match state.mode {
                    TimerMode::Work => {
                        state.pomodoros_completed += 1;
                        if state.pomodoros_completed % 4 == 0 {
                            state.mode = TimerMode::LongBreak;
                            state.seconds_remaining = state.long_break_duration * 60;
                            title = "Work Session Complete!";
                            body = "Time for a long break.";
                        } else {
                            state.mode = TimerMode::ShortBreak;
                            state.seconds_remaining = state.short_break_duration * 60;
                            title = "Work Session Complete!";
                            body = "Time for a short break.";
                        }
                        // Break starts automatically
                        state.is_running = true;
                    }
                    TimerMode::ShortBreak | TimerMode::LongBreak => {
                        state.mode = TimerMode::Work;
                        state.seconds_remaining = state.work_duration * 60;
                        title = "Break Over!";
                        body = "Ready to focus again?";
                        // Work needs manual start
                        state.is_running = false;
                    }
                }

                // Send Notification
                let _ = app
                    .notification()
                    .builder()
                    .title(title)
                    .body(body)
                    .show();
            }
        }

        emit_state(&state, &app);
    });
}

fn main() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppStateMutex(Mutex::new(AppState::new())))
        .setup(|app| {
            let app_handle = app.handle().clone();
            start_background_thread(app_handle);

            let quit_i = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = tauri::menu::MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app: &AppHandle, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            reset_timer,
            set_durations,
            set_mode
        ]);

    builder.build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            RunEvent::WindowEvent { label, event: WindowEvent::CloseRequested { api, .. }, .. } => {
                let app_handle = app_handle.clone();
                let window = app_handle.get_webview_window(&label).unwrap();
                let _ = window.hide();
                api.prevent_close();
            }
            _ => {}
        });
}
