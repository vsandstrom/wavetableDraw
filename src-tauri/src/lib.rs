use tauri::{State};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct TableValue {
  pub value: f32,
  pub index: usize
}

pub struct EnvValue {
  pub value: f32,
  pub index: usize
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn trigger(value: f32, state: State<SynthControl>) {
  let _ = state.trig_tx.send(value);
}

#[tauri::command]
// fn send_table(value: f32, index: usize, state: State<TableValueTX>) {
fn update_table(value: f32, index: usize, state: State<SynthControl>) {
  let _ = state.table_tx.send(TableValue{value, index});
}

#[tauri::command]
fn set_frequency(value: f32, state: State<SynthControl>) {
  let _ = state.freq_tx.send(value);
}

#[tauri::command]
fn set_volume(value: f32, state: State<SynthControl>) {
  let _ = state.vol_tx.send(value);
}

#[tauri::command]
fn set_interpolation(value: usize, state: State<SynthControl>) {
  let _ = state.lerp_tx.send(value);
}

#[tauri::command]
fn set_attack(value: f32, state: State<SynthControl>) {
  let _ = state.env_tx.send(EnvValue{value, index: 0});
}

#[tauri::command]
fn set_release(value: f32, state: State<SynthControl>) {
  let _ = state.env_tx.send(EnvValue{value, index: 1});
}

pub struct SynthControl {
  pub trig_tx: Sender<f32>,
  pub table_tx: Sender<TableValue>,
  pub vol_tx: Sender<f32>,
  pub freq_tx: Sender<f32>,
  pub lerp_tx: Sender<usize>,
  pub env_tx: Sender<EnvValue>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(ctrl: SynthControl) -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(ctrl)
        .invoke_handler(tauri::generate_handler![
          trigger,
          update_table,
          set_volume,
          set_frequency,
          set_interpolation,
          set_attack,
          set_release

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
