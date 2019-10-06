pub struct Tree {
    nodes: std::vec::Vec<std::boxed::Box<dyn super::core_widget::Widget>>
}

impl Tree {

    pub fn new() -> Tree {
        return Tree {
            nodes: vec!()
        };
    }

    pub fn branch(&self) -> &Tree {
        return self;
    }

    pub fn climb(&self) -> &Tree {
        return self;
    }
}