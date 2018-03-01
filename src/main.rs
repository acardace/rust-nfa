extern crate nfa;

use nfa::nfa_algorithms::*;
use std::env::*;

fn main() {
    if let Some(re) = args().nth(1) {
        if let Ok(nfa) = re_to_nfa(&re) {
            let e_closure = nfa.epsilon_closure(&0);
            println!("{:?}", nfa);
            println!("{:?}", e_closure);
        }
    } else {
        println!("Usage: {:?} RE", current_exe().unwrap().file_name().unwrap());
    }
}
