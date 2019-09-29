pub trait Widget {
    fn render(&self) -> super::core_tree::Tree;
}

