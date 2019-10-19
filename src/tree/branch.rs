use crate::widget::Widget;
use std::boxed::Box;
use std::vec::Vec;

pub struct TreeBranch {
    children: Vec<TreeBranch>,
    host: Box<dyn Widget>
}

impl TreeBranch {

    pub fn new(host: Box<dyn Widget>) -> TreeBranch {
        TreeBranch {
            children: Vec::new(),
            host
        }
    }
}