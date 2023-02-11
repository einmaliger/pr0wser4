#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::read;
use tauri::http::{method::Method,ResponseBuilder};
mod scenedatabase;
mod tokenizer;

#[tauri::command]
fn load(path: &str) -> Vec<scenedatabase::Scene> {
  let database: String = std::fs::read_to_string(path).expect("File could not be opened");
  let mut t = tokenizer::Tokenizer::new(database.as_bytes());
  match scenedatabase::parse_database(&mut t) {
      Ok(f) => f.film,
      Err(e) => panic!("Error while parsing {}: {}", "input file", e)
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load])
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
