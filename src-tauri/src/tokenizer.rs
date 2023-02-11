/// A very simple/naive tokenizer that splits an u8 slice into tokens of the following types:
/// * Token::EOF    - end of the input reached, or immediately after Tokenizer::new before anything has been processed
/// * Token::NUMBER - a sequence of digits, interpreted as base10 number
/// * Token::STRING - either a sequence of alphanumeric characters, starting with a letter,
///                   or something that is enclosed in quotation marks. These can go across lines
///                   and include anything except quotation marks. No escape
/// * Token::CHAR   - any other character
///
/// Usage:
/// Initialize with Tokenizer::new(&u8)
///
/// Access the token with Tokenizer::t
///
/// Tokenizer::next - advance to the next token.
/// (Note: Must be executed once after Tokenizer::new to get to the first token.)
//
/// Tokenizer::eat(&Token) - advance to the next token if the current token equals the argument,
///                          or return an error.

#[derive(PartialEq)]
pub enum Token<'a> {
    EOF,
    NUMBER(i32),
    STRING(&'a str),
    CHAR(u8),
}

// A self-contained token type, suitable to print/display.
// When converting to it Strings are truncated
// to 10 characters.
pub enum PrintableToken {
    EOF,
    NUMBER(i32),
    STRING(String),
    CHAR(std::ascii::EscapeDefault),
}

impl From<&Token<'_>> for PrintableToken {
    fn from(t: &Token) -> PrintableToken {
        return match *t {
            Token::EOF => PrintableToken::EOF,
            Token::NUMBER(n) => PrintableToken::NUMBER(n),
            Token::STRING(s) => 
                PrintableToken::STRING(
                    if s.len() <= 10 {
                        String::from(s)
                    } else {
                            String::from(&s[0..7]) + "..."
                    }
                ),
            Token::CHAR(c) => PrintableToken::CHAR(std::ascii::escape_default(c))
        }
    }
}

impl std::fmt::Display for PrintableToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PrintableToken::EOF => write!(f, "<EOF>"),
            PrintableToken::NUMBER(n) => write!(f, "number <{}>", n),
            PrintableToken::STRING(s) => write!(f, "\"{}\"", s),
            PrintableToken::CHAR(c) => write!(f, "'{}'", c),
        }
    }
}

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pub t: Token<'a>,       // current token must life as long as the input as it may contain a string slice to it
    index: usize,
}

// An error when something is read from the Tokenizer, but there is nothing more to read.
pub struct UnexpectedEndError;

// An error when the current token is not the one that was expected (method eat)
pub struct ExpectedTokenError {
    pub expected: PrintableToken,
    pub found: PrintableToken
}

impl ExpectedTokenError {
    pub fn new(s: &Token, t: &Token) -> Self {
        ExpectedTokenError {
            expected: PrintableToken::from(s),
            found: PrintableToken::from(t)
        }
    }
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Expected {}, found {}", self.expected, self.found)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tokenizer {
            input,
            t: Token::EOF,
            index: 0,
        }
    }

    // If t is a string token, it is tied to self, so its' lifetime is tied to our's
    pub fn eat(&mut self, t: &'a Token) -> Result<(), ExpectedTokenError> {
        if self.t == *t {
            match self.next() {
                Ok(()) => Ok(()),
                Err(UnexpectedEndError) => Err(ExpectedTokenError::new(t, &Token::EOF))
            }
        } else {
            Err(ExpectedTokenError::new(t, &self.t))
        }
    }

    // Puts the next token into self.t
    pub fn next(&mut self) -> Result<(), UnexpectedEndError> {
        loop {
            if self.index >= self.input.len() {
                self.t = Token::EOF;
                break Ok(())
            }

            let c = self.getc()?;
            match char::from(c) {
                c if c.is_whitespace() => continue,
                '#' => { // skip a comment (until the end of the line)
                    while self.getc()? != b'\n' {}
                },
                c if c.is_alphanumeric() => { // read a word or number = sequence of alnum
                    let is_number = c.is_digit(10);
                    let word_start = self.index-1;
                    loop {
                        let c = char::from(self.getc()?);
                        if !c.is_alphanumeric() { self.index -= 1; break; }
                    }
                    let s = std::str::from_utf8(&self.input[word_start..self.index]).unwrap();
                    if is_number {self.t = Token::NUMBER(s.parse().unwrap());}
                    else {self.t = Token::STRING(s);}
                    break Ok(());
                },
                '"' => { // read a quoted string
                    let string_start = self.index;
                    loop {
                        let c = char::from(self.getc()?);
                        if c == '"' {break;}
                    }
                    let s = std::str::from_utf8(&self.input[string_start..self.index-1]).unwrap(); // index-1 since we exclude the quotation marks
                    self.t = Token::STRING(s);
                    break Ok(());
                }
                _ => { self.t = Token::CHAR(c); break Ok(()); }
            }
        }
    }

    fn getc(&mut self) -> Result<u8, UnexpectedEndError> {
        if self.index >= self.input.len() {
            Err(UnexpectedEndError)
        } else {
            self.index += 1;
            Ok(self.input[self.index-1])
        }
    }
}

