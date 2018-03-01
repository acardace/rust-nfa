#[derive(Debug)]
#[derive(Clone)]
pub struct Nfa {
    vertexes: Vec<u32>,
    transitions: Vec<(u32, char, u32)>,
}

impl Nfa {
    pub fn new(c: char) -> Nfa {
        Nfa { vertexes: vec![0, 1], transitions: vec![(0, c, 1)] }
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

    pub fn concatenate(&mut self, operand: &Nfa) {
        // add middle ε transition and vertex
        let last_vertex = *self.vertexes.last().unwrap();
        self.vertexes.push(last_vertex + 1);
        self.transitions.push((last_vertex, 'ε', last_vertex + 1));

        // concat vertexes
        let last_vertex = *self.vertexes.last().unwrap();
        for vertex in operand.vertexes.iter() {
            self.vertexes.push(*vertex + last_vertex + 1);
        }
        // concat nfa copying transition function
        for &(s, c, e) in operand.transitions.iter() {
            self.transitions.push((s + last_vertex + 1, c, e + last_vertex + 1));
        }
    }

    pub fn kleene(&mut self) {
        let start_state = self.vertexes[0];
        let final_state = *self.vertexes.last().unwrap();
        self.transitions.push((final_state, 'ε', start_state));

        self.shift_right(); 
        let start_state = self.vertexes[0];
        self.vertexes.insert(0, start_state - 1);
        self.transitions.push((start_state - 1, 'ε', start_state));

        let start_state = self.vertexes[0];
        let final_state = *self.vertexes.last().unwrap();
        self.vertexes.push(final_state + 1);
        self.transitions.push((start_state, 'ε', final_state + 1));
        self.transitions.push((final_state, 'ε', final_state + 1));
    }

    pub fn or(&mut self, operand: &Nfa) {
        self.shift_right(); 
        // add new initial state
        let self_start_state = self.vertexes[0];
        self.vertexes.insert(0, self_start_state - 1);
        // add ε moves
        self.transitions.push((0, 'ε', self.vertexes[1]));
        let self_final_state = *self.vertexes.last().unwrap();
        // add second NFA
        let last_vertex = self_final_state + 1;
        for v in operand.vertexes.iter() {
            self.vertexes.push(*v + last_vertex);
        }
        for tran in operand.transitions.iter() {
            self.transitions.push((tran.0 + last_vertex, tran.1, tran.2 + last_vertex));
        }
        // add ε move
        self.transitions.push((0, 'ε', operand.vertexes[0] + last_vertex));
        let op_final_state = *self.vertexes.last().unwrap();
        // add final state
        let nfa_final_state = op_final_state + 1;
        self.vertexes.push(nfa_final_state);
        self.transitions.push((self_final_state, 'ε', nfa_final_state));
        self.transitions.push((op_final_state, 'ε', nfa_final_state));
    }
}

impl Default for Nfa {
    fn default() -> Nfa {
        Nfa { vertexes: vec![], transitions: vec![] }
    }
}
