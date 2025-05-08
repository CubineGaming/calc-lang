#[derive(Clone, Debug)]
pub enum Token {
    Int(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}
pub fn tokenize(input: &String) -> Vec<Token> {
    let mut tokens:Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let word = String::new();
    let mut position = 0;
    let mut char;

    while position < String::len(input) {
        char = chars.next().unwrap_or_else(|| panic!("Unexpected end of input"));
        match char {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ' ' => continue, // Skip whitespace
            _ => if char.is_digit(10) {
                tokens.push(Token::Int(char.to_digit(10).unwrap() as i32));
            }
        }
        position += 1;
    }
    tokens
    
}
