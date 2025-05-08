use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expr()
    }

    fn eat(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let mut node = self.term()?;

        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus => {
                    self.eat();
                    node = Expr::BinOp(Box::new(node), Op::Add, Box::new(self.term()?));
                }
                Token::Minus => {
                    self.eat();
                    node = Expr::BinOp(Box::new(node), Op::Sub, Box::new(self.term()?));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut node = self.factor()?;

        while let Some(tok) = self.peek() {
            match tok {
                Token::Multiply => {
                    self.eat();
                    node = Expr::BinOp(Box::new(node), Op::Mul, Box::new(self.factor()?));
                }
                Token::Divide => {
                    self.eat();
                    node = Expr::BinOp(Box::new(node), Op::Div, Box::new(self.factor()?));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        match self.eat() {
            Some(Token::Int(n)) => Ok(Expr::Num(*n)),
            Some(Token::LParen) => {
                let expr = self.expr()?;
                if let Some(Token::RParen) = self.eat() {
                    Ok(expr)
                } else {
                    Err("Expected ')'".into())
                }
            }
            _ => Err("Expected number or '('".into()),
        }
    }
}

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::BinOp(lhs, op, rhs) => {
            let l = eval(lhs);
            let r = eval(rhs);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    }
}
