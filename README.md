# rust-nfa
Learning rust by playing around with NFAs

# Build NFA from REs (Thompson's construction)
```
extern crate nfa;

use nfa::algorithms::*;

fn main() {
    if let Ok(nfa) = re_to_nfa("ab*(c|d)") {
        println!("{:?}", nfa);
    }
}
```
