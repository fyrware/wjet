pub struct Lexer {
    conditions: std::collections::VecDeque<bool>
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            conditions: std::collections::VecDeque::with_capacity(10)
        }
    }

    pub fn condition(&self) -> Option<&bool> {
        self.conditions.back()
    }

    pub fn start_condition(&mut self, condition: bool) {
        self.conditions.push_back(condition);
    }

    pub fn end_condition(&mut self) {
        self.conditions.pop_back();
    }
}