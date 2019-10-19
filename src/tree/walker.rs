use crate::tree::Tree;
use crate::tree::branch::TreeBranch;
use std::vec::Vec;

fn climb_from(base: &mut TreeBranch) -> &mut TreeBranch {

    if base.branches().len() == 0 {
        return base
    }

    climb_from(base.branches().last_mut().unwrap())
}

pub struct TreeWalker<'refer> {
    target: &'refer mut Tree
}

impl TreeWalker<'_> {

    pub fn new(target: &mut Tree) -> TreeWalker {
        TreeWalker {
            target
        }
    }

    pub fn climb(&mut self, depth: u8) -> Option<&mut TreeBranch> {
        let branches = self.target.branches();

        if branches.len() == 0 {
            return None
        }

        let base = branches.last_mut().unwrap();

        Some(climb_from(base))
    }
}