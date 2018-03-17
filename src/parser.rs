use nfa::Nfa;
use lexer::Token;

/* Grammar
   regex -> A
   A    -> B [|] A | B | ε
   B    -> C B | C
   C    -> D*  | D
   D    -> (A) | [^|*()]
 */

/* produce AST using shunting yard algorithm */

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    nfa_stack: Vec<Nfa>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(t: &Vec<Token>) -> Parser {
        Parser {
            tokens: t,
            nfa_stack: vec![],
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<&Nfa, String> {
        self.regex();
        if let Some(t) = self.peek() {
            Err(format!(
                "Parse error at index {}, found {:?}",
                self.index, t
            ))
        } else {
            Ok(self.nfa_stack.last().unwrap())
        }
    }

    fn regex(&mut self) {
        self.a()
    }

    fn a(&mut self) {
        self.b();
        if let Some(&Token::Or) = self.peek() {
            self.consume();
            self.a();
            // println!("OR");
            // process OR
            let op2 = self.nfa_stack.pop().unwrap();
            let mut op1 = self.nfa_stack.pop().unwrap();
            op1.or(&op2);
            self.nfa_stack.push(op1);
        }
    }

    fn b(&mut self) {
        self.c();
        if let Some(t) = self.peek().cloned() {
            if t != Token::ParClose && t != Token::Or && t != Token::Kleene {
                self.b();
                // println!("CONCAT");
                // process concatenation
                let op2 = self.nfa_stack.pop().unwrap();
                let mut op1 = self.nfa_stack.pop().unwrap();
                op1.concatenate(&op2);
                self.nfa_stack.push(op1);
            }
        }
    }

    fn c(&mut self) {
        self.d();
        if let Some(&Token::Kleene) = self.peek() {
            self.consume();
            // println!("KLEENE");
            // process kleene
            let mut op1 = self.nfa_stack.pop().unwrap();
            op1.kleene();
            self.nfa_stack.push(op1);
        }
    }

    fn d(&mut self) {
        match self.peek() {
            Some(&Token::ParOpen) => {
                self.consume();
                self.a();
                if let Some(&Token::ParClose) = self.peek() {
                    self.consume();
                } else {
                    panic!("Error in parsing at {}, expected )", self.index,);
                }
            }
            Some(&Token::Char(c)) => {
                self.consume();
                self.nfa_stack.push(Nfa::new(c));
                // println!("char {}", c);
            }
            _ => panic!(
                "Error in parsing at {}, token {:?}, expected [^)|* eof]",
                self.index,
                self.peek()
            ),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }
}
