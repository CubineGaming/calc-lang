use crate::lexer;
use crate::parser;

#[test]
fn int_token_check() {
  assert_eq!(lexer::tokenize("5"), vec![lexer::Token::Int(5)]);
}
