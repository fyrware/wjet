pub struct Tree {
    nodes: std::vec::Vec<std::boxed::Box<dyn super::core_widget::Widget>>
}

impl Tree {

    pub fn new() -> Tree {
        Tree {
            nodes: vec!()
        }
    }
}