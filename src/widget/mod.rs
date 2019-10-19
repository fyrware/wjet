use crate::tree::Tree;

pub trait Widget {
    fn render(&mut self) -> Tree;
}