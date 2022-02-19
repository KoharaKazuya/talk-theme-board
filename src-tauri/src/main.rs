#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod parser;

use crate::parser::{load_file, load_text};
use tauri::{api::dialog, CustomMenuItem, Menu, MenuItem, Submenu};

const CHEAT_SHEET: &str = "Cmd + Shift + o: クリップボードから開く\nCmd + Shift + j: 次のテーマ\nCmd + Shift + k: 前のテーマ\nCmd + Shift + Control + c: 終了する";

fn main() {
  let menu = Menu::new().add_submenu(Submenu::new(
    "app",
    Menu::new()
      .add_item(CustomMenuItem::new("help", "ヘルプ"))
      .add_native_item(MenuItem::Quit),
  ));

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
      if event.menu_item_id() == "help" {
        dialog::message(Some(event.window()), "ヘルプ", CHEAT_SHEET);
      }
    })
    .invoke_handler(tauri::generate_handler![load_text, load_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
