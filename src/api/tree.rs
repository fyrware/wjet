use super::Widget;

pub struct Tree {

}

impl Tree {

    pub fn new() -> Tree {
        Tree {

        }
    }

    fn foo(&self) {

    }

    fn bar(&self) {
        self.foo()
    }
}