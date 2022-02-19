#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod parser;

use crate::parser::{load_file, load_text};
use tauri::{Menu, MenuItem, Submenu};

fn main() {
  let menu = Menu::new().add_submenu(Submenu::new(
    "app",
    Menu::new().add_native_item(MenuItem::Quit),
  ));

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![load_text, load_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
