pub trait Widget {
    fn attach(&self) -> super::State;
    fn design(&self) -> super::Style;
    fn render(&self, children: super::Tree) -> super::Tree;
}