#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// -------------------------------------
// Define Controller API
// -------------------------------------
use enigo::{Enigo, MouseControllable, KeyboardControllable, Key, MouseButton};

#[tauri::command]
fn enigo_test() {
  let mut enigo = Enigo::new();
  println!("screen dimensions: {:?}", enigo.main_display_size());
  println!("mouse location: {:?}", enigo.mouse_location());
  enigo.mouse_move_to(500, 200);
}

#[tauri::command]
fn mouse_move_to(x:i32, y:i32) {
  let mut enigo = Enigo::new();
  enigo.mouse_move_to(x, y);
}

fn get_button(button: &str) -> MouseButton{
  match button {
    "left" => MouseButton::Left,
    "right" => MouseButton::Right,
    _ => MouseButton::Left
  }
}

#[tauri::command]
fn mouse_click(button: &str) {
  let mut enigo = Enigo::new();
  enigo.mouse_click(get_button(button));
}

fn get_key(key: &str) -> Key{
  match key {
    "f1"  => Key::F1,
    "f2"  => Key::F2,
    "f3"  => Key::F3,
    "f4"  => Key::F4,
    "f5"  => Key::F5,
    "f6"  => Key::F6,
    "f7"  => Key::F7,
    "f8"  => Key::F8,
    "f9"  => Key::F9,
    "f10" => Key::F10,
    "f11" => Key::F11,
    "f12" => Key::F12,
    "cmd" => Key::Meta,
    "option" => Key::Option,
    "shift" => Key::Shift,
    "ctrl" => Key::Control,
    "alt" => Key::Alt,
    _ => Key::Layout(key.chars().next().unwrap()),
  }
}

#[tauri::command]
fn key_down(key: &str) {
  let mut enigo = Enigo::new();
  enigo.key_down(get_key(key));
}

#[tauri::command]
fn key_up(key: &str) {
  let mut enigo = Enigo::new();
  enigo.key_up(get_key(key));
}

#[tauri::command]
fn key_click(key: &str) {
  let mut enigo = Enigo::new();
  enigo.key_click(get_key(key));
}

// -------------------------------------
// Define Controller High-end API
// -------------------------------------
#[tauri::command]
fn copy() {
  let mut enigo = Enigo::new();
  enigo.key_down(Key::Control);
  enigo.key_click(Key::C);
  enigo.key_up(Key::Control);
}

#[tauri::command]
fn quick_shifting() {
  let mut enigo = Enigo::new();
  enigo.key_down(Key::Control);
  enigo.mouse_click(MouseButton::Left);
  enigo.key_up(Key::Control);
}

// -------------------------------------
// Main
// -------------------------------------
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      enigo_test, mouse_move_to, mouse_click, key_down, key_up, key_click,
      copy, quick_shifting
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
