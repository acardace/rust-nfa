extern crate nfa;

use nfa::algorithms::*;

fn main() {
    let re = "a|bc*";
    let nfa = re_to_nfa(&re);
    println!("{:?}", nfa);
}
