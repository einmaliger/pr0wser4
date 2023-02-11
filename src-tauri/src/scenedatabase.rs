use std::collections::HashMap;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::tokenizer;
use tokenizer::{PrintableToken,Token};


#[derive(Deserialize,Serialize)]
pub struct SceneDatabase {
    pub base_dir: String,
    pub film: Vec<Scene>,
}

#[derive(Deserialize,Serialize)]
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
    pub length: Option<String>,
    pub num_girls: i32,
    pub num_boys: i32,
}

impl Scene {
    fn from_tokenizer(t: &'_ mut tokenizer::Tokenizer, id: i32) -> Result<Scene, ParserError> {
        type Maps = (HashMap<String, String>, HashMap<String, i32>);

        // loop to get a list of key-value pairs.
        let mut val: Maps = (HashMap::new(), HashMap::new());

        loop {
            let key: String;
            match &t.t {
                Token::CHAR(b'}') => { break; },
                Token::STRING(s) => { key = s.to_string(); },
                _ => { return Result::Err(expected_string_error("<key>", &t.t)); }
            }
            t.next()?; t.eat(&Token::CHAR(b':'))?;
            match &t.t {                
                Token::STRING(s) => {val.0.insert(key, s.to_string());}
                Token::NUMBER(n) => {val.1.insert(key, *n);}
                _ => { return Result::Err(expected_string_error("<value>", &t.t)); }
            }
            t.next()?;
        }

        let extract_string = | val: &mut Maps, s: &str | -> Result<String, KeyNotFoundError> {
            match val.0.remove(s) {
                Some(s) => Ok(s),
                None => Err(KeyNotFoundError{key: String::from(s)}),
            }
        };

        let extract_number = | val: &mut Maps, s: &str | -> Result<i32, KeyNotFoundError> {
            match val.1.remove(s) {
                Some(i) => Ok(i),
                None => Err(KeyNotFoundError{key: String::from(s)}),
            }
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
            },
            None => {},
        }

        Ok( Scene {
            id,
            file_name: extract_string(&mut val, "fileName")?,
            name: val.0.remove("name"),
            directory: extract_string(&mut val, "directory")?,
            thumb_file_name: val.0.remove("thumbFileName"),
            website: val.0.remove("webSite"),
            actors: val.0.remove("actors"),
            cmd_parm: val.0.remove("cmdParm"),
            tags,
            begin: val.1.remove("begin"),
            end: val.1.remove("end"),
            year: val.1.remove("year"),
            length: val.0.remove("length"),
            num_girls: extract_number(&mut val, "numGirls")?,
            num_boys: extract_number(&mut val, "numBoys")?,
        } )
    }
}

pub fn parse_database(mut t: &'_ mut tokenizer::Tokenizer) -> Result<SceneDatabase, ParserError> {
    t.next()?;
    t.eat(&Token::STRING("type"))?;
    t.eat(&Token::CHAR(b':'))?;
    t.eat(&Token::STRING("pr0wser3"))?;
    t.eat(&Token::STRING("baseDir"))?;
    t.eat(&Token::CHAR(b':'))?;

    let base_dir: String;

    match &t.t {
        Token::STRING(s) => { base_dir = s.to_string() },
        _ => { return Result::Err(expected_string_error("<base directory>", &t.t)); } 
    }

    t.next()?;

    let mut films = Vec::<Scene>::new();

    let mut id: i32 = 0;

    while Token::EOF != t.t {
        t.eat(&Token::CHAR(b'{'))?;
        match Scene::from_tokenizer(&mut t, id) {
            Ok(film) => { films.push(film); },
            Err(e) => { return Err(e); }
        }
        t.eat(&Token::CHAR(b'}'))?;
        id += 1;
        if id < 0  {
            panic!("Too many scenes in database (i32 overflow)");
        }
    }

    Ok(SceneDatabase{base_dir, film: films})
}



// Emitted if a required value was not found in the fields of a film description
// s is the key that was not found
struct KeyNotFoundError { key: String }

pub enum ParserError {
    UnexpectedEnd,
    ExpectedToken(tokenizer::ExpectedTokenError),
    KeyNotFound(String),
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
            KeyNotFound(key) => write!(f, "No value was set for {}", key),
        }
    }
}

// Shortcut to create a ParserError::ExpectedToken for a string token that was not found
fn expected_string_error(s: &str, t: &Token) -> ParserError {
    ParserError::ExpectedToken(
        tokenizer::ExpectedTokenError {
            expected: PrintableToken::STRING(s.to_string()),
            found: PrintableToken::from(t),
        }
    )
}
