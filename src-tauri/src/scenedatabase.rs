use crate::{
  tokenizer,
  tokenizer::{PrintableToken, Token},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct SceneDatabase {
  pub base_dir: String,
  pub film: Vec<Scene>,
  pub def_search: Option<String>,
  pub tags: Vec<(String, String)>, // may be empty, since this is optional
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
  pub id: i32,
  pub file_name: String,
  pub name: Option<String>,
  pub directory: String,
  pub thumb_file_name: Option<String>,
  pub website: Option<String>,
  pub actors: Option<String>,
  pub cmd_parm: Option<String>,
  pub tags: HashSet<String>,
  pub begin: Option<i32>,
  pub end: Option<i32>,
  pub year: Option<i32>,
  pub length: Option<i32>,
  pub num_girls: i32,
  pub num_boys: i32,
  pub score: i32,
  pub notes: Option<String>,
}

type OrderedMaps = (Vec<(String, String)>, Vec<(String, i32)>);
type Maps = (HashMap<String, String>, HashMap<String, i32>);

fn from_unordered(o: &OrderedMaps) -> Maps {
  let mut result: Maps = (
    HashMap::<String, String>::new(),
    HashMap::<String, i32>::new(),
  );
  for x in &(o.0) {
    result.0.insert(x.0.clone(), x.1.clone());
  }
  for x in &(o.1) {
    result.1.insert(x.0.clone(), x.1);
  }
  result
}

fn get_object(t: &'_ mut tokenizer::Tokenizer) -> Result<OrderedMaps, ParserError> {
  // loop to get a list of key-value pairs.
  let mut val: OrderedMaps = (Vec::<(String, String)>::new(), Vec::<(String, i32)>::new());

  loop {
    let key: String;
    match &t.t {
      Token::CHAR(b'}') => {
        break;
      }
      Token::STRING(s) => {
        key = s.to_string();
      }
      _ => {
        return Result::Err(expected_string_error("<key>", &t.t));
      }
    }
    t.next()?;
    t.eat(&Token::CHAR(b':'))?;
    match &t.t {
      Token::STRING(s) => {
        val.0.push((key, s.to_string()));
      }
      Token::NUMBER(n) => {
        val.1.push((key, *n));
      }
      _ => {
        return Result::Err(expected_string_error("<value>", &t.t));
      }
    }
    t.next()?;
  }

  return Result::Ok(val);
}

impl Scene {
  fn from_tokenizer(t: &'_ mut tokenizer::Tokenizer) -> Result<Scene, ParserError> {
    let orderedval = get_object(t)?;

    let mut val = from_unordered(&orderedval);

    // TODO:
    // The following three functions take val as parameter, which is always the val of this enclosing function.
    // There should be a way to omit this parameter. Right now, this fails because of borrow rules that I seem to not
    // understand sufficiently to fix this.
    let extract_string = |val: &mut Maps, s: &str| -> Result<String, KeyNotFoundError> {
      match val.0.remove(s) {
        Some(s) => Ok(s),
        None => Err(KeyNotFoundError {
          key: String::from(s),
        }),
      }
    };

    let extract_number = |val: &mut Maps, s: &str| -> Result<i32, KeyNotFoundError> {
      match val.1.remove(s) {
        Some(i) => Ok(i),
        None => Err(KeyNotFoundError {
          key: String::from(s),
        }),
      }
    };

    let get_time = |val: &mut Maps, s: &str| -> Option<i32> {
      // If it's a number, return it immediately
      let value = val.1.remove(s);
      if value.is_some() {
        return value;
      }

      // If it's neither a number, nor a string, return None
      let value = val.0.remove(s);
      if value.is_none() {
        return None; // Cannot return value here as it is Option<String>!
      }

      // It's a string, so convert it to a number
      let value = value.unwrap(); // TODO: understand why I need this temporary variable
      let parts = value.split(':');

      let mut result: i32 = 0;
      for s in parts {
        // TODO: Instead of panicking, the from_tokenizer function
        // should return an appropriate error.
        // Possible approach:
        // - get_time returns a Result<Option<i32>>
        // - the parts below that call it, need to be rearranged
        //   to first take into account possible errors while storing
        //   the results in temporary variables and then construct th
        //   Ok() object.
        result = result * 60 + s.parse::<i32>().expect("invalid time format");
      }

      Some(result)
    };

    // Build tags HashSet
    let tags_string = extract_string(&mut val, "tags")?;
    let tags_vec: Vec<&str> = tags_string.split(",").collect();
    let mut tags = HashSet::new();
    for s in tags_vec {
      tags.insert(s.trim().to_string());
    }

    // Convert the shortcut field "basic"
    match val.1.remove("basic") {
      Some(n) => {
        val.1.insert(String::from("year"), n);
        val.1.insert(String::from("numGirls"), 1);
        val.1.insert(String::from("numBoys"), 1);
      }
      None => {}
    }

    Ok(Scene {
      id: -1,
      file_name: extract_string(&mut val, "fileName")?,
      name: val.0.remove("name"),
      directory: extract_string(&mut val, "directory")?,
      thumb_file_name: val.0.remove("thumbFileName"),
      website: val.0.remove("webSite"),
      actors: val.0.remove("actors"),
      cmd_parm: val.0.remove("cmdParm"),
      tags,
      begin: get_time(&mut val, "begin"),
      end: get_time(&mut val, "end"),
      year: val.1.remove("year"),
      length: get_time(&mut val, "length"),
      num_girls: extract_number(&mut val, "numGirls")?,
      num_boys: extract_number(&mut val, "numBoys")?,
      score: val.1.remove("score").unwrap_or(0),
      notes: val.0.remove("notes"),
    })
  }
}

pub fn parse_database(mut t: &'_ mut tokenizer::Tokenizer) -> Result<SceneDatabase, ParserError> {
  let mut known_format = false;
  let mut base_dir = String::new();
  let mut def_search: Option<String> = None;
  let mut tags = Vec::<(String, String)>::new();

  t.next()?;

  while t.t != Token::EOF {
    let key = match &t.t {
      Token::STRING(s) => s.to_string(),
      _ => break,
    };

    // Expect something of the form key : stringvalue or key : {object} (only for tags)
    t.next()?;
    t.eat(&Token::CHAR(b':'))?;
    let value = match &t.t {
      Token::STRING(s) => s.to_string(),
      Token::CHAR(b'{') => {
        t.next()?;
        tags = get_object(t)?.0; // TODO: if something in .1, this should throw an error

        String::from("tags object")
      }
      _ => {
        return Result::Err(ParserError::ExpectedString);
      }
    };

    t.next()?;

    match key.as_str() {
      "type" => {
        if value != "pr0wser3" && value != "pr0wser4" {
          return Result::Err(ParserError::UnknownFileFormat);
        }
        known_format = true;
      }
      "baseDir" => {
        base_dir = value;
      }
      "defSearch" => {
        def_search = Some(value);
      }
      "tags" => {} // already handled above
      _ => return Result::Err(ParserError::UnknownKey(key)),
    }
  }

  if !known_format {
    return Result::Err(ParserError::UnknownFileFormat);
  }
  if base_dir.is_empty() {
    return Result::Err(ParserError::NoBaseDir);
  }

  let mut films = Vec::<Scene>::new();

  while Token::EOF != t.t {
    t.eat(&Token::CHAR(b'{'))?;
    match Scene::from_tokenizer(&mut t) {
      Ok(film) => {
        films.push(film);
      }
      Err(e) => {
        return Err(e);
      }
    }
    t.eat(&Token::CHAR(b'}'))?;
  }

  // Set the index such that the last entry has index 1,
  // the one before it has index 2, and so on
  // (useful for the "latest:" filter)
  let size = films.len();
  for i in 0..size {
    films[i].id = (size - i) as i32;
  }

  Ok(SceneDatabase {
    base_dir,
    film: films,
    def_search,
    tags,
  })
}

// Emitted if a required value was not found in the fields of a film description
// s is the key that was not found
struct KeyNotFoundError {
  key: String,
}

pub enum ParserError {
  UnexpectedEnd,
  ExpectedToken(tokenizer::ExpectedTokenError),
  ExpectedString,
  KeyNotFound(String),
  UnknownKey(String),
  UnknownFileFormat,
  NoBaseDir,
}

impl From<tokenizer::UnexpectedEndError> for ParserError {
  fn from(_err: tokenizer::UnexpectedEndError) -> ParserError {
    ParserError::UnexpectedEnd
  }
}

impl From<tokenizer::ExpectedTokenError> for ParserError {
  fn from(err: tokenizer::ExpectedTokenError) -> ParserError {
    match err.found {
      PrintableToken::EOF => ParserError::UnexpectedEnd,
      _ => ParserError::ExpectedToken(err),
    }
  }
}

impl From<KeyNotFoundError> for ParserError {
  fn from(err: KeyNotFoundError) -> ParserError {
    ParserError::KeyNotFound(err.key)
  }
}

impl std::fmt::Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use ParserError::*;

    match &*self {
      UnexpectedEnd => write!(f, "Unexpected end of file"),
      ExpectedToken(ref e) => e.fmt(f),
      ExpectedString => write!(f, "String value expected"),
      KeyNotFound(key) => write!(f, "No value was set for {}", key),
      UnknownKey(key) => write!(f, "Unknown assignment {}", key),
      UnknownFileFormat => write!(f, "Unknown file format"),
      NoBaseDir => write!(f, "No baseDir given"),
    }
  }
}

// Shortcut to create a ParserError::ExpectedToken for a string token that was not found
fn expected_string_error(s: &str, t: &Token) -> ParserError {
  ParserError::ExpectedToken(tokenizer::ExpectedTokenError {
    expected: PrintableToken::STRING(s.to_string()),
    found: PrintableToken::from(t),
  })
}
