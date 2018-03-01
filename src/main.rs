extern crate nfa;

use nfa::algorithms::*;
use std::env::*;

fn main() {
    if let Some(re) = args().nth(1) {
        match re_to_nfa(&re) {
            Ok(nfa) => println!("{:?}", nfa),
            Err(s) => println!("{}", s),
        }
    } else {
        println!("Usage: {:?} RE", current_exe().unwrap().file_name().unwrap());
    }
}
