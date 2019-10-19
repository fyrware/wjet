use crate::tree::Tree;
use crate::tree::branch::TreeBranch;
use std::vec::Vec;

fn climb_to_height(branches: Option<&mut Vec<TreeBranch>>, height: u8) -> &mut Vec<TreeBranch> {
    match branches {
        Some => {
            let exists = branches.unwrap();

            if exists.len() == 0 {
                exists
            }

            climb_to_height(exists.get(exists.len() - 1).children(), height)
        },
        None => {
            ()
        }
    }
}

pub struct TreeWalker<'a> {
    target: &'a mut Tree
}

impl TreeWalker<'_> {

    pub fn new(target: &mut Tree) -> TreeWalker {
        TreeWalker {
            target
        }
    }

    pub fn dive(&mut self, depth: u8) -> &mut Vec<TreeBranch> {
        climb_to_height(Some(self.target.roots()), depth)
    }
}