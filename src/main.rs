mod lexer;
mod parser;

use crate::parser::eval;
use std::io;

fn main() {
  println!("Enter your program:");
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
      Ok(_goes_into_input_above) => {},
      Err(_no_updates_is_fine) => {},
  }
  let tokens = lexer::tokenize(&input);
  let mut parser = parser::Parser::new(&tokens);
  let ast = parser.parse().unwrap();
  println!("Result: {}", eval(&ast));
}
