use nfa::Nfa;

// TODO support ()
pub fn re_to_nfa(re: &str) -> Nfa {
    let mut nfa: Nfa = Default::default();
    let mut or_op = false;
    let mut escape_op = false;
    for c in re.chars() {
        match c {
            _ if escape_op => {
                escape_op = false;
                nfa.concatenate(&Nfa::new(c));
            }
            _ if or_op => {
                or_op = false;
                nfa.or(&Nfa::new(c));
            }
            '|' => or_op = true,
            '*' => nfa.kleene(),
            '/' => escape_op = true,  
            _ => nfa.concatenate(&Nfa::new(c)),
        }
    }   
    nfa
}
