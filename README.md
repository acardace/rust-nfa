# rust-nfa
Learning rust by playing around with NFAs

# Build NFA from REs (Thompson's construction)
```rust
let re = "ab|c*"
let tokens = Lexer::new(&re).lex();
let mut parser = Parser::new(&tokens);
match parser.parse() {
    Ok(nfa) => {
        println!("{:?}", nfa);
    }
    Err(s) => println!("{}", s),
}
```
