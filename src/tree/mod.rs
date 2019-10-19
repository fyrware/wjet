pub mod branch;
pub mod walker;

use crate::tree::branch::TreeBranch;
use crate::tree::walker::TreeWalker;
use crate::widget::Widget;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

static UNNAMED_BRANCH: &str = "";

pub struct Tree {
    children: Vec<TreeBranch>,
    depth: u8
}

impl Tree {

    pub fn new() -> Tree {
        Tree {
            children: Vec::new(),
            depth: 0
        }
    }

    pub fn branches(&mut self) -> &mut Vec<TreeBranch> {
        &mut self.children
    }

    pub fn branch(&mut self, host: Rc<dyn Widget>, name: String) {

        if name != UNNAMED_BRANCH.to_string() {
            let clone = host.clone();
        }

        let depth = self.depth;
        let mut walker = TreeWalker::new(self);
        let mut value = TreeBranch::new(host);

        if depth == 0 {
            self.children.push(value);
        }
        else {
            let mut base = walker.climb(depth);

            match base {
                Some(_) => {
                    base.unwrap().branches().push(value);
                },
                None => {
                    // I dunno, ask me later; probably nothing
                }
            }
        }

        self.depth += 1;
    }

    pub fn climb(&mut self) {
        self.depth -= 1;
    }
}