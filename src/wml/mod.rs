use crate::core::core_tree;

pub mod wml_lexer;
pub mod wml_token;

#[macro_use]
mod wml_markup;

#[macro_export]
macro_rules! wml {

    ($($wml:tt)*) => {
        {
            let mut wml_lexer = wml_lexer::Lexer::new();
            let mut wml_tree = core_tree::Tree::new();

            wml_markup!(|wml_lexer, wml_tree| -> ($($wml)*));

            return wml_tree;
        }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Implementation:
////////////////////////////////////////////////////////////////////////////////////////////////////

struct Block {
    pub foo: bool
}

impl Block {
    fn new() -> Self {
        Block {
            foo: true
        }
    }
}

impl crate::core::core_widget::Widget for Block {

    fn render(&self) -> core_tree::Tree {
        unimplemented!()
    }
}

fn render(condition: bool) -> core_tree::Tree {
    let bar = false;

    return wml! {
       <Block { foo: bar }>
            <{ if condition }>
                <Block { foo: bar };>
            <{ else }>
                <{ "Hello World!" };>
            <;>
        <Block;>
        <Block { foo: condition }>
            <{ for x in 0..9 }>
                <{ x };>
            <;>
        <Block;>
    }
}

#[test]
fn oof() {
    render(true);
}