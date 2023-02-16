#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::read;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use url::Url;
use tauri::http::{method::Method,ResponseBuilder};
use scenedatabase::SceneDatabase;
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

fn absolute_directory(base_dir: &str, directory: &str) -> PathBuf {
  let mut p = PathBuf::new();

  #[cfg(target_os = "windows")]
  let base_dir = base_dir.replace("/", "\\");

  #[cfg(target_os = "windows")]
  let directory = directory.replace("/", "\\");

  p.push(base_dir);
  p.push(directory);    // Note that if this is absolute, it replaces base_dir

  p
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
    // handler inspired by https://medium.com/@marm.nakamura/practice-rust-and-tauri-make-an-image-viewer-4-39623547b06d
    .register_uri_scheme_protocol("screenshot", move | _app, request | {

      const DEFAULT_IMAGE: &str = "../static/nothumb.jpg";

      let response = ResponseBuilder::new();

      if request.method() != Method::GET {
        return response.status(400).body(Vec::new());
      }

      let query: HashMap<_, _> = Url::parse(request.uri()).unwrap().query_pairs().into_owned().collect();

      static EMPTY_STRING: String = String::new();

      let base_dir = query.get("base_dir").unwrap();
      let directory = query.get("directory").unwrap();
      let file_name = query.get("file_name").unwrap_or(&EMPTY_STRING);
      let thumb_file_name = query.get("thumb_file_name").unwrap_or(&EMPTY_STRING);

      let mut p: PathBuf; // path to the screenshot image file
      let mut mime_type = "image/jpeg"; // mimetype of the screenshot image

      if file_name.is_empty() && thumb_file_name.is_empty() {
        // no useful information given - return the default image
        p =  PathBuf::from(DEFAULT_IMAGE);
      } else {
        p = absolute_directory(base_dir, directory);

        p.push(".pr0wser");

        if !thumb_file_name.is_empty() {
          p.push(thumb_file_name);
        } else {
            // If no thumb file name is given, try to figure out an existing image
            // by appending various known image file extension to the file_name
            p.push(String::from(file_name) + ".jpg");
            if !p.exists() { p.set_extension("jpeg"); }
            if !p.exists() { p.set_extension("png"); mime_type = "image/png"; }
            if !p.exists() { p.set_extension("webp"); mime_type = "image/webp"; }
            if !p.exists() { p.set_extension("gif"); mime_type = "image/gif"; }
            if !p.exists() { p = PathBuf::from(DEFAULT_IMAGE); }
        }
      }

      let image = if let Ok(data) = read(p) {
        response.mimetype(mime_type).body(data)
      } else {
        response.status(404).body(Vec::new())
      };

      image

    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
