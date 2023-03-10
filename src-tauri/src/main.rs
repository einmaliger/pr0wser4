#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod dbfiles;
mod scenedatabase;
mod tokenizer;

use crate::{
  dbfiles::{absolute_directory, screenshot_handler},
  scenedatabase::SceneDatabase,
};

use std::process::Command;

#[tauri::command]
fn load(path: &str) -> SceneDatabase {
  let database: String = std::fs::read_to_string(path).expect("File could not be opened");
  let mut t = tokenizer::Tokenizer::new(database.as_bytes());
  match scenedatabase::parse_database(&mut t) {
    Ok(f) => f,
    Err(e) => panic!("Error while parsing {}: {}", "input file", e),
  }
}

#[tauri::command]
fn play(base_dir: &str, directory: &str, file_name: &str) {
  let mut p = absolute_directory(base_dir, directory);

  p.push(file_name);

  println!("Running mpv to play file \"{}\"", p.to_string_lossy());

  Command::new("C:\\Program Files\\mpv\\mpv")
    .arg(p)
    .output()
    .expect("Could not run mpv");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load, play])
    .register_uri_scheme_protocol("screenshot", screenshot_handler)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
