use crate::tree::Tree;
use crate::widget::Widget;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;
use std::fmt::Result;
use std::vec::Vec;

pub struct TreeBranch {
    children: Vec<TreeBranch>,
    host: Rc<dyn Widget>
}

impl TreeBranch {

    pub fn new(host: Rc<dyn Widget>) -> TreeBranch {
        TreeBranch {
            children: Vec::new(),
            host
        }
    }

    pub fn branches(&mut self) -> &mut Vec<TreeBranch> {
        &mut self.children
    }
}