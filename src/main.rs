extern crate nfa;

use nfa::nfa::Nfa;

fn main() {
    let mut a = Nfa::new('a');
    let b = Nfa::new('b');
    a.concatenate(&b);
    a.or(&b);
    a.kleene();
    println!("{:?}", a);
}
