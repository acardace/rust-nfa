#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod lexer;
pub mod parser;
pub mod nfa;
pub mod nfa_algorithms;
