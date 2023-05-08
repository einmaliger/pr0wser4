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
fn load(path: &str, handle: tauri::AppHandle) -> SceneDatabase {
  let default_file = handle
    .path_resolver()
    .resolve_resource("../static/test.pr0")
    .unwrap();
  let dbfile = match path {
    "" => default_file.to_str().unwrap(),
    _ => path,
  };
  let database: String = std::fs::read_to_string(dbfile).expect("File could not be opened");
  let mut t = tokenizer::Tokenizer::new(database.as_bytes());
  match scenedatabase::parse_database(&mut t) {
    Ok(f) => f,
    Err(e) => panic!("Error while parsing {}: {}", "input file", e),
  }
}

#[tauri::command]
fn play(base_dir: &str, directory: &str, file_name: &str, begin: i32, length: i32) {
  let mut p = absolute_directory(base_dir, directory);

  p.push(file_name);

  println!(
    "Running mpv to play file \"{}\" (begin: {}, length: {})",
    p.to_string_lossy(),
    begin,
    length
  );

  let mut cmd = Command::new("C:\\Program Files\\mpv\\mpv");
  let mut cmd = cmd.arg(p);

  if begin > 0 {
    cmd = cmd.arg(format!("--start={}", begin));
  }

  if length > 0 {
    cmd = cmd.arg(format!("--length={}", length));
  }

  cmd.output().expect("Could not run mpv");
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![load, play])
    .register_uri_scheme_protocol("screenshot", screenshot_handler)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
