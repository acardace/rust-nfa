extern crate nfa;

use nfa::lexer::Lexer;
use nfa::parser::Parser;
use std::env::*;

fn main() {
    if let Some(re) = args().nth(1) {
        let tokens = Lexer::new(&re).lex();
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(nfa) => {
                println!("{:?}", nfa);
            }
            Err(s) => println!("{}", s),
        }
    } else {
        println!(
            "Usage: {:?} RE",
            current_exe().unwrap().file_name().unwrap()
        );
    }
}
