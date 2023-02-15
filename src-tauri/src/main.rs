#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::read;
use std::path::PathBuf;
use std::process::Command;
use scenedatabase::SceneDatabase;
use tauri::http::{method::Method,ResponseBuilder};
mod scenedatabase;
mod tokenizer;

#[tauri::command]
fn load(path: &str) -> SceneDatabase {
  let database: String = std::fs::read_to_string(path).expect("File could not be opened");
  let mut t = tokenizer::Tokenizer::new(database.as_bytes());
  match scenedatabase::parse_database(&mut t) {
      Ok(f) => f,
      Err(e) => panic!("Error while parsing {}: {}", "input file", e)
  }
}



#[tauri::command]
fn play(base_dir: &str, directory: &str, file_name: &str) {
  let mut p: PathBuf = PathBuf::new();

  #[cfg(target_os = "windows")]
  let base_dir = base_dir.replace("/", "\\");

  #[cfg(target_os = "windows")]
  let directory = directory.replace("/", "\\");

  p.push(base_dir);
  p.push(directory);    // Note that if this is absolute, it replaces base_dir
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
    // handler inspired by https://medium.com/@marm.nakamura/practice-rust-and-tauri-make-an-image-viewer-4-39623547b06d
    .register_uri_scheme_protocol("thumbnail", move | app, request | {
      let response = ResponseBuilder::new();

      if request.method() != Method::GET {
        return response.status(400).body(Vec::new());
      }

      let nothumb = app.path_resolver().resolve_resource("../static/nothumb.jpg").unwrap();

      // Right now, this only returns a default image*/
      let image = if let Ok(data) = read(nothumb) {
        response.mimetype("image/jpg").body(data)
      } else {
        response.status(404).body(Vec::new())
      };

      image

    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
