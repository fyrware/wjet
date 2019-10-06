pub struct Lexer {
    conditions: std::collections::VecDeque<bool>,
    grammar: std::collections::VecDeque<super::wml_token::Token>,
    iterators: std::collections::VecDeque<u32>
}

impl Lexer {

    pub fn new() -> Lexer {
        return Lexer {
            conditions: std::collections::VecDeque::new(),
            grammar: std::collections::VecDeque::new(),
            iterators: std::collections::VecDeque::new()
        };
    }

    pub fn condition(&self) -> Option<&bool> {
        return self.conditions.back();
    }

    pub fn grammar(&self) -> Option<&super::wml_token::Token> {
        return self.grammar.back();
    }

    pub fn start_condition(&mut self, condition: bool) {
        self.conditions.push_back(condition);
    }

    pub fn end_condition(&mut self) {
        self.conditions.pop_back();
    }

    pub fn start_grammar(&mut self, grammar: super::wml_token::Token) {
        self.grammar.push_back(grammar);
    }

    pub fn end_grammar(&mut self) {
        self.grammar.pop_back();
    }
}