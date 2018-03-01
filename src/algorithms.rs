use nfa::Nfa;
use std::str::Chars;

pub fn re_to_nfa(re: &str) -> Result<Nfa, &'static str> {
    sub_re_to_nfa(&mut re.chars(), false)
}

fn sub_re_to_nfa(iter: &mut Chars, par: bool) -> Result<Nfa, &'static str> {
    let mut prev = ' ';
    let mut nfa: Nfa = Default::default();
    let mut or_op = false;
    let mut escape_op = false;
    while let Some(c) = iter.next() {
        match c {
            _ if escape_op => {
                escape_op = false;
                if or_op {
                    or_op = false;
                    nfa.or(&Nfa::new(c));
                } else {
                    nfa.concatenate(&Nfa::new(c));
                }
            }
            ')' => {
                if par {
                    return Ok(nfa);
                } else {
                    return Err("Parse error -- unbalanced ')'");
                }
            }
            '(' => {
                match sub_re_to_nfa(iter, true) {
                    Ok(sub_nfa) => nfa.concatenate(&sub_nfa),
                    Err(err) => return Err(err),
                }
            }
            '|' => {
                if or_op {
                    return Err("Parse error -- double ||");
                } else {
                    or_op = true;
                }
            }
            _ if or_op => {
                or_op = false;
                nfa.or(&Nfa::new(c));
            }
            '*' => {
                if prev != '*' {
                    nfa.kleene();
                } else {
                    return Err("Parse error -- double **");
                }
            }
            '/' => escape_op = true,  
            _ => nfa.concatenate(&Nfa::new(c)),
        }
        prev = c;
    }
    if par {
        Err("Parse error -- no closing ')'")
    } else {
        Ok(nfa)
    }
}
