#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Window};
use std::{thread, time};

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn clear() -> String {
    format!("")
}

#[tauri::command]
fn update_y(count: f64) -> Vec<f64> {
    let mut y:Vec<f64> = Vec::new();
    for i in 0..100 {
        let f = i as f64;
        y.push((f*0.1+(count/(2.0*3.14))).sin());
    }
    y
}

#[tauri::command]
fn update_x() -> Vec<f64> {
    let mut x:Vec<f64> = Vec::new();
    for i in 0..100 {
        let f = i as f64;
        x.push(f);
    }
    x
}

#[tauri::command]
fn init_process(window: Window) {
    println!("Thread started");
    std::thread::spawn(move || {
        let mut count = 0;
        loop {
            window.emit("update", Payload {message: "Event!".into()} ).unwrap();
            thread::sleep(time::Duration::from_millis(100));
            println!("Looping: {}", count);
            count += 1;
            if count > 100 {
                break;
            }
        }
    });
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, clear, init_process, update_x, update_y])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
