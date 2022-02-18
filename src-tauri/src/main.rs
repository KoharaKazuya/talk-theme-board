#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod parser;

use crate::parser::load;
use tauri::{Menu, MenuItem, Submenu};

fn main() {
  let menu = Menu::new().add_submenu(Submenu::new(
    "app",
    Menu::new().add_native_item(MenuItem::Quit),
  ));

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![load])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
