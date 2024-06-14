use tauri::{AppHandle, State};
use std::sync::{Mutex, Arc, mpsc::Sender};

#[derive(Debug)]
pub struct TableValue {
  pub value: f32,
  pub index: usize
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn trigger(value: bool, state: State<TriggerTX>) {
  state.tx.send(value);
}

#[tauri::command]
fn send_table(value: f32, index: usize, state: State<TableValueTX>) {
  state.tx.send(TableValue{value, index});
}

struct TriggerTX {
  tx: Sender<bool>
}

struct TableValueTX {
  tx: Sender<TableValue>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(tx: Sender<bool>, table_tx: Sender<TableValue>) -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(TriggerTX{tx})
        .manage(TableValueTX{tx: table_tx})
        .invoke_handler(tauri::generate_handler![
          greet,
          trigger,
          send_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
