use std::collections::HashSet;

pub type Vertex = u32;

#[derive(Debug, Clone, Default)]
pub struct Transition(Vertex, char, Vertex);

impl Transition {
    pub fn new(s: Vertex, c: char, e: Vertex) -> Transition {
        Transition { 0: s, 1: c, 2: e }
    }
    pub fn get_char(&self) -> char {
        self.1
    }
}

#[derive(Debug, Clone, Default)]
pub struct Nfa {
    vertexes: Vec<Vertex>,
    transitions: Vec<Transition>,
}

impl Nfa {
    pub fn new(c: char) -> Nfa {
        Nfa {
            vertexes: vec![0, 1],
            transitions: vec![Transition(0, c, 1)],
        }
    }

    pub fn transitions(&self) -> &Vec<Transition> {
        &self.transitions
    }

    pub fn get_start_state(&self) -> Option<&Vertex> {
        self.vertexes.first()
    }

    pub fn get_final_state(&self) -> Option<&Vertex> {
        self.vertexes.last()
    }

    fn shift_right(&mut self) {
        // shift everything by 1
        for tran in self.transitions.iter_mut() {
            tran.0 += 1;
            tran.2 += 1;
        }
        for v in self.vertexes.iter_mut() {
            *v += 1;
        }
    }

    pub fn concatenate(&mut self, op: &Nfa) {
        // add middle ε transition and vertex
        if self.vertexes.len() > 0 && op.vertexes.len() > 0 {
            let offset = *self.vertexes.last().unwrap() + 1;
            // concat 2nd Nfa
            for v in &op.vertexes {
                self.vertexes.push(*v + offset);
            }
            // connect the two
            self.transitions.push(Transition(offset - 1, 'ε', offset));
            for &Transition(s, c, e) in &op.transitions {
                self.transitions.push(Transition(s + offset, c, e + offset));
            }
        } else if op.vertexes.len() > 0 {
            *self = op.clone();
        }
    }

    pub fn kleene(&mut self) {
        if self.vertexes.len() > 0 {
            let start_state = self.vertexes[0];
            let final_state = *self.vertexes.last().unwrap();
            self.transitions
                .push(Transition(final_state, 'ε', start_state));

            self.shift_right();
            let start_state = self.vertexes[0];
            self.vertexes.insert(0, start_state - 1);
            self.transitions
                .push(Transition(start_state - 1, 'ε', start_state));

            let start_state = self.vertexes[0];
            let final_state = *self.vertexes.last().unwrap();
            self.vertexes.push(final_state + 1);
            self.transitions
                .push(Transition(start_state, 'ε', final_state + 1));
            self.transitions
                .push(Transition(final_state, 'ε', final_state + 1));
        }
    }

    pub fn or(&mut self, operand: &Nfa) {
        if self.vertexes.len() > 0 {
            self.shift_right();
            // add new initial state
            let self_start_state = self.vertexes[0];
            self.vertexes.insert(0, self_start_state - 1);
            // add ε moves
            self.transitions.push(Transition(0, 'ε', self.vertexes[1]));
            let self_final_state = *self.vertexes.last().unwrap();
            // add second NFA
            let last_vertex = self_final_state + 1;
            for v in operand.vertexes.iter() {
                self.vertexes.push(*v + last_vertex);
            }
            for tran in operand.transitions.iter() {
                self.transitions.push(Transition(
                    tran.0 + last_vertex,
                    tran.1,
                    tran.2 + last_vertex,
                ));
            }
            // add ε move
            self.transitions
                .push(Transition(0, 'ε', operand.vertexes[0] + last_vertex));
            let op_final_state = *self.vertexes.last().unwrap();
            // add final state
            let nfa_final_state = op_final_state + 1;
            self.vertexes.push(nfa_final_state);
            self.transitions
                .push(Transition(self_final_state, 'ε', nfa_final_state));
            self.transitions
                .push(Transition(op_final_state, 'ε', nfa_final_state));
        }
    }

    pub fn epsilon_closure(&self, states: &Vec<Vertex>) -> Vec<Vertex> {
        let mut closure: HashSet<Vertex> = HashSet::new();
        closure.extend(states);

        for &state in states {
            let mut explored: Vec<Vertex> = Vec::new();
            let mut unexplored = vec![state];

            while let Some(v) = unexplored.pop() {
                explored.push(v);
                for &Transition(s, c, e) in self.transitions.iter() {
                    if s == v && c == 'ε' && !explored.contains(&e) {
                        closure.insert(e);
                        unexplored.push(e);
                    }
                }
            }
        }
        closure.into_iter().collect()
    }

    pub fn delta(&self, states: &Vec<Vertex>, transition: &Transition) -> Vec<Vertex> {
        let mut delta = vec![];
        for &state in states {
            let &Transition(s, _, v) = transition;
            if s == state {
                delta.push(v);
            }
        }
        delta
    }
}
