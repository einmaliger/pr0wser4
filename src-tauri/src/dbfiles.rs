use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{
  http::ResponseBuilder,
  http::{method::Method, Request},
  AppHandle,
};
use url::Url;

// Determine the absolute directory by combining base_dir and directory
pub fn absolute_directory(base_dir: &str, directory: &str) -> PathBuf {
  let mut p = PathBuf::new();

  #[cfg(target_os = "windows")]
  let base_dir = base_dir.replace("/", "\\");

  #[cfg(target_os = "windows")]
  let directory = directory.replace("/", "\\");

  p.push(base_dir);
  p.push(directory); // Note that if this is absolute, it replaces base_dir

  p
}

// handler inspired by https://medium.com/@marm.nakamura/practice-rust-and-tauri-make-an-image-viewer-4-39623547b06d
pub fn screenshot_handler(
  app: &AppHandle,
  request: &Request,
) -> Result<tauri::http::Response, Box<dyn std::error::Error>> {
  const DEFAULT_IMAGE: &str = "../static/nothumb.jpg";

  let response = ResponseBuilder::new();

  if request.method() != Method::GET {
    return response.status(400).body(Vec::new());
  }

  let query: HashMap<_, _> = Url::parse(request.uri())
    .unwrap()
    .query_pairs()
    .into_owned()
    .collect();

  static EMPTY_STRING: String = String::new();

  let base_dir = query.get("base_dir").unwrap();
  let directory = query.get("directory").unwrap();
  let file_name = query.get("file_name").unwrap_or(&EMPTY_STRING);
  let thumb_file_name = query.get("thumb_file_name").unwrap_or(&EMPTY_STRING);

  let mut p: PathBuf; // path to the screenshot image file
  let mut mime_type = "image/jpeg"; // mimetype of the screenshot image

  if file_name.is_empty() && thumb_file_name.is_empty() {
    // no useful information given - return the default image
    p = app.path_resolver().resolve_resource(DEFAULT_IMAGE).unwrap();
  } else {
    p = absolute_directory(base_dir, directory);

    p.push(".pr0wser");

    if !thumb_file_name.is_empty() {
      p.push(thumb_file_name);
    } else {
      // If no thumb file name is given, try to figure out an existing image
      // by appending various known image file extension to the file_name
      p.push(String::from(file_name) + ".jpg");
      if !p.exists() {
        p.set_extension("jpeg");
      }
      if !p.exists() {
        p.set_extension("png");
        mime_type = "image/png";
      }
      if !p.exists() {
        p.set_extension("webp");
        mime_type = "image/webp";
      }
      if !p.exists() {
        p.set_extension("gif");
        mime_type = "image/gif";
      }
      if !p.exists() {
        p = PathBuf::from(DEFAULT_IMAGE);
      }
    }
  }

  let image = if let Ok(data) = std::fs::read(p) {
    response.mimetype(mime_type).body(data)
  } else {
    response.status(404).body(Vec::new())
  };

  image
}
