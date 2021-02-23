use std::env;
use std::fmt;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Debug)]
#[derive(Copy, Clone)]
enum TokenType {
  Ampersand,
  At,
  Asterisk,
  Backslash,
  Backtick,
  CarriageReturn,
  Colon,
  Comma,
  CurlyBracketClose,
  CurlyBracketOpen,
  Digit,
  Dot,
  Dollar,
  DoubleQuotes,
  Equals,
  EOF,
  Exclamation,
  ForwardSlash,
  GT,
  Hashtag,
  LT,
  Letter,
  Minus,
  Newline,
  ParenthesisClose,
  ParenthesisOpen,
  Pipe,
  Plus,
  QuestionMark,
  Semicolon,
  SingleQuote,
  Space,
  SquareBracketClose,
  SquareBracketOpen,
  Tab,
  Tilde,
  Uncategorized,
  Underscore,
  Word,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

#[derive(Debug)]
#[derive(Clone)]
struct Token(TokenType, String, usize);

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = format!("test_files/{}", &args[1]);

  println!("Tokenizing {}", filename);

  let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
  let result = tokenize_contents(&contents);

  print_to_file(filename, result);
}

fn tokenize_contents(contents: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();

  for (i, character) in contents.chars().enumerate() {
    match character {
      // Special characters
      '&' => tokens.push(Token(TokenType::Ampersand, String::from(character), i)),
      '@' => tokens.push(Token(TokenType::At, String::from(character), i)),
      '*' => tokens.push(Token(TokenType::Asterisk, String::from(character), i)),
      '`' => tokens.push(Token(TokenType::Backtick, String::from(character), i)),
      '\\' => tokens.push(Token(TokenType::Backslash, String::from(character), i)),
      ':' => tokens.push(Token(TokenType::Colon, String::from(character), i)),
      ',' => tokens.push(Token(TokenType::Comma, String::from(character), i)),
      '}' => tokens.push(Token(TokenType::CurlyBracketClose, String::from(character), i)),
      '{' => tokens.push(Token(TokenType::CurlyBracketOpen, String::from(character), i)),
      '.' => tokens.push(Token(TokenType::Dot, String::from(character), i)),
      '$' => tokens.push(Token(TokenType::Dollar, String::from(character), i)),
      '"' => tokens.push(Token(TokenType::DoubleQuotes, String::from(character), i)),
      '=' => tokens.push(Token(TokenType::Equals, String::from(character), i)),
      '!' => tokens.push(Token(TokenType::Exclamation, String::from(character), i)),
      '/' => tokens.push(Token(TokenType::ForwardSlash, String::from(character), i)),
      '>' => tokens.push(Token(TokenType::GT, String::from(character), i)),
      '#' => tokens.push(Token(TokenType::Hashtag, String::from(character), i)),
      '<' => tokens.push(Token(TokenType::LT, String::from(character), i)),
      '-' => tokens.push(Token(TokenType::Minus, String::from(character), i)),
      ')' => tokens.push(Token(TokenType::ParenthesisClose, String::from(character), i)),
      '(' => tokens.push(Token(TokenType::ParenthesisOpen, String::from(character), i)),
      '|' => tokens.push(Token(TokenType::Pipe, String::from(character), i)),
      '+' => tokens.push(Token(TokenType::Plus, String::from(character), i)),
      '?' => tokens.push(Token(TokenType::QuestionMark, String::from(character), i)),
      ';' => tokens.push(Token(TokenType::Semicolon, String::from(character), i)),
      '\'' => tokens.push(Token(TokenType::SingleQuote, String::from(character), i)),
      ']' => tokens.push(Token(TokenType::SquareBracketClose, String::from(character), i)),
      '[' => tokens.push(Token(TokenType::SquareBracketOpen, String::from(character), i)),
      '~' => tokens.push(Token(TokenType::Tilde, String::from(character), i)),
      '_' => tokens.push(Token(TokenType::Underscore, String::from(character), i)),
      // Whitespace
      ' ' => tokens.push(Token(TokenType::Space, String::from(character), i)),
      '\n' => tokens.push(Token(TokenType::Newline, String::from(character), i)),
      '\t' => tokens.push(Token(TokenType::Tab, String::from(character), i)),
      '\r' => tokens.push(Token(TokenType::CarriageReturn, String::from(character), i)),
      // Letters
      'A'..='Z' | 'a'..='z' => tokens.push(Token(TokenType::Letter, String::from(character), i)),
      '0'..='9' => tokens.push(Token(TokenType::Digit, String::from(character), i)),
      // Everything else
      _ => tokens.push(Token(TokenType::Uncategorized, String::from(character), i)),
    }
  }

  tokens.push(Token(TokenType::EOF, String::from(""), contents.chars().count()));

  return process_letters_into_words(tokens);
}

fn process_letters_into_words(tokenized_characters: Vec<Token>) -> Vec<Token> {
  let mut processed_tokens: Vec<Token> = Vec::new();
  let mut start_of_word: usize = 0;
  let mut word: String = "".to_owned();

  for (i, token) in tokenized_characters.iter().enumerate() {
    match token {
      Token(TokenType::Letter, contents, _) => {
        if word.len() == 0 {
          start_of_word = i
        }

        word.push_str(contents)
      },
      _ => {
        if start_of_word > 0 || word.len() > 0 {
          processed_tokens.push(Token(TokenType::Word, word, start_of_word));
          word = "".to_owned();
          start_of_word = 0;
        }
        processed_tokens.push(token.clone());
      }
    }
  }

  return processed_tokens;
}

fn print_to_file(filename: String, tuples: Vec<Token>) {
  let file = fs::File::create(format!("{}.txt", filename)).expect("Unable to create file");
  let mut file_writer = BufWriter::new(file);

  for tuple in tuples {
    write!(&mut file_writer, "{:?}\n", tuple).expect("Error while writing to file");
  }
}
