pub mod branch;
pub mod walker;

use crate::tree::branch::TreeBranch;
use crate::tree::walker::TreeWalker;
use crate::widget::Widget;
use std::boxed::Box;
use std::vec::Vec;

pub struct Tree {
    branches: Vec<TreeBranch>,
    depth: u8
}

impl Tree {

    pub fn new() -> Tree {
        Tree {
            branches: Vec::new(),
            depth: 0
        }
    }

    pub fn roots(&mut self) -> &mut Vec<TreeBranch> {
        &mut self.branches
    }

    pub fn branch(&mut self, host: Box<dyn Widget>) {
        let mut walker = TreeWalker::new(self);
        let mut branches = walker.dive(depth);
        branches.push(TreeBranch::new(host));

        self.depth += 1;
    }

    pub fn climb(&mut self) {
        self.depth -= 1;
    }
}