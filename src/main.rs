use std::env;
use std::fmt;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Debug)]
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
  Hashtag,
  Minus,
  Newline,
  ParenthesisClose,
  ParenthesisOpen,
  Plus,
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

  let result = tokenize_contents(&contents);

  print_to_file(filename, result);
}

fn tokenize_contents(contents: &str) -> Vec<(TokenType, String, usize)> {
  let mut tokens: Vec<(TokenType, String, usize)> = Vec::new();
  let mut word: Vec<char> = Vec::new();
  let mut current_word_length = 0;
  let mut word_index = 0;

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
      '#' => tokens.push((TokenType::Hashtag, String::from(character), i)),
      '-' => tokens.push((TokenType::Minus, String::from(character), i)),
      ')' => tokens.push((TokenType::ParenthesisClose, String::from(character), i)),
      '(' => tokens.push((TokenType::ParenthesisOpen, String::from(character), i)),
      '+' => tokens.push((TokenType::Plus, String::from(character), i)),
      ']' => tokens.push((TokenType::SquareBracketClose, String::from(character), i)),
      '[' => tokens.push((TokenType::SquareBracketOpen, String::from(character), i)),
      '_' => tokens.push((TokenType::Underscore, String::from(character), i)),
      // Whitespace
      ' ' => tokens.push((TokenType::Space, String::from(character), i)),
      '\n' => tokens.push((TokenType::Newline, String::from(character), i)),
      '\t' => tokens.push((TokenType::Tab, String::from(character), i)),
      '\r' => tokens.push((TokenType::CarriageReturn, String::from(character), i)),
      // Everything else
      _ => {
        if word.len() == 0 {
          word_index = i
        }
        word.push(character);
      }
    }

    let length = word.len();
    let has_current_word = length > 0;
    let processed_special_character = current_word_length == length;

    if has_current_word && processed_special_character {
      // There will always be at least one item in the vector
      let special_token = tokens.pop().unwrap();
      tokens.push((TokenType::Word, word.iter().collect(), word_index));
      tokens.push(special_token);
      // Reset word variables
      word.clear();
      current_word_length = 0;
    } else {
      current_word_length = length;
    }
  }

  tokens.push((TokenType::EOF, String::from(""), contents.chars().count()));

  return tokens;
}

fn print_to_file(filename: String, tuples: Vec<(TokenType, String, usize)>) {
  let file = fs::File::create(format!("{}.txt", filename)).expect("Unable to create file");
  let mut file_writer = BufWriter::new(file);

  for tuple in tuples {
    write!(&mut file_writer, "{:?}\n", tuple).expect("Error while writing to file");
  }
}
