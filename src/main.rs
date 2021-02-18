use std::env;
use std::fmt;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Debug)]
#[derive(Copy, Clone)]
enum TokenType {
  Asterisk,
  Backslash,
  Backtick,
  CarriageReturn,
  CurlyBracketClose,
  CurlyBracketOpen,
  Dot,
  DoubleQuotes,
  EOF,
  Exclamation,
  Hashtag,
  Letter,
  Minus,
  Newline,
  ParenthesisClose,
  ParenthesisOpen,
  Plus,
  SingleQuote,
  Space,
  SquareBracketClose,
  SquareBracketOpen,
  Tab,
  Underscore,
  Word,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = format!("test_files/{}", &args[1]);

  println!("In file {}", filename);

  let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
  let result = process_words(tokenize_contents(&contents));

  print_to_file(filename, result);
}

fn tokenize_contents(contents: &str) -> Vec<(TokenType, String, usize)> {
  let mut tokens: Vec<(TokenType, String, usize)> = Vec::new();

  for (i, character) in contents.chars().enumerate() {
    match character {
      // Special characters
      '*' => tokens.push((TokenType::Asterisk, String::from(character), i)),
      '`' => tokens.push((TokenType::Backtick, String::from(character), i)),
      '\\' => tokens.push((TokenType::Backslash, String::from(character), i)),
      '}' => tokens.push((TokenType::CurlyBracketClose, String::from(character), i)),
      '{' => tokens.push((TokenType::CurlyBracketOpen, String::from(character), i)),
      '.' => tokens.push((TokenType::Dot, String::from(character), i)),
      '"' => tokens.push((TokenType::DoubleQuotes, String::from(character), i)),
      '!' => tokens.push((TokenType::Exclamation, String::from(character), i)),
      '#' => tokens.push((TokenType::Hashtag, String::from(character), i)),
      '-' => tokens.push((TokenType::Minus, String::from(character), i)),
      ')' => tokens.push((TokenType::ParenthesisClose, String::from(character), i)),
      '(' => tokens.push((TokenType::ParenthesisOpen, String::from(character), i)),
      '+' => tokens.push((TokenType::Plus, String::from(character), i)),
      '\'' => tokens.push((TokenType::SingleQuote, String::from(character), i)),
      ']' => tokens.push((TokenType::SquareBracketClose, String::from(character), i)),
      '[' => tokens.push((TokenType::SquareBracketOpen, String::from(character), i)),
      '_' => tokens.push((TokenType::Underscore, String::from(character), i)),
      // Whitespace
      ' ' => tokens.push((TokenType::Space, String::from(character), i)),
      '\n' => tokens.push((TokenType::Newline, String::from(character), i)),
      '\t' => tokens.push((TokenType::Tab, String::from(character), i)),
      '\r' => tokens.push((TokenType::CarriageReturn, String::from(character), i)),
      // Everything else
      _ => tokens.push((TokenType::Letter, String::from(character), i)),
    }
  }

  tokens.push((TokenType::EOF, String::from(""), contents.chars().count()));

  return tokens;
}

fn process_words(tokenized_characters: Vec<(TokenType, String, usize)>) -> Vec<(TokenType, String, usize)> {
  let mut processed_tokens: Vec<(TokenType, String, usize)> = Vec::new();
  let mut start_of_word: usize = 0;
  let mut word: String = "".to_owned();

  for (i, token) in tokenized_characters.iter().enumerate() {
    match token {
      (TokenType::Letter, contents, _) => {
        if word.len() == 0 {
          start_of_word = i
        }

        word.push_str(contents)
      },
      _ => {
        if start_of_word > 0 || word.len() > 0 {
          processed_tokens.push((TokenType::Word, word, start_of_word));
          word = "".to_owned();
          start_of_word = 0;
        }
        let (token_type, contents, index) = token;
        processed_tokens.push((*token_type, contents.clone(), *index));
      }
    }
  }

  return processed_tokens;
}

fn print_to_file(filename: String, tuples: Vec<(TokenType, String, usize)>) {
  let file = fs::File::create(format!("{}.txt", filename)).expect("Unable to create file");
  let mut file_writer = BufWriter::new(file);

  for tuple in tuples {
    write!(&mut file_writer, "{:?}\n", tuple).expect("Error while writing to file");
  }
}
