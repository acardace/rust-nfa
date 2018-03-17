#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    None,
    Char(char),
    Concat,
    Kleene,
    Or,
    ParOpen,
    ParClose,
}

pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &str) -> Lexer {
        Lexer { source: s }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens = vec![];
        for c in self.source.chars() {
            match c {
                '|' => tokens.push(Token::Or),
                '*' => tokens.push(Token::Kleene),
                '(' => tokens.push(Token::ParOpen),
                ')' => tokens.push(Token::ParClose),
                _ => tokens.push(Token::Char(c)),
            }
        }
        tokens
    }
}
