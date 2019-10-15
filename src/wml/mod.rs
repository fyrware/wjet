use crate::core::core_tree;

pub mod wml_lexer;
pub mod wml_token;

#[macro_use]
mod wml_markup;

#[macro_export]
macro_rules! wml {

    ($($wml:tt)*) => {
        {
            let mut wjet_lexer = wml_lexer::Lexer::new();
            let mut wjet_tree = core_tree::Tree::new();
            let mut wjet_widget = ();

            wml_markup!(|wjet_tree, wjet_lexer, wjet_widget| -> ($($wml)*));

            wjet_tree
        }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Implementation:
////////////////////////////////////////////////////////////////////////////////////////////////////

struct Block {
    pub foo: bool,
    pub baz: u8
}

impl Block {

    fn new() -> Block {
         Block {
            foo: true,
            baz: 0
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

    wml! {
        <Block> {
            let foo = false;
            let name = "World";

            if (!condition) {
                <Block { foo: condition, baz: 7 }>
            } else if (foo) {
                <Block { foo: true }>
            } else {
                println!("Hello {}", name);
            }
            <Block { baz: 72 }>
        }
        <Block {}> {
            // This will render the content 10 times
            for x in (0..10) {
                <Block { foo: x > 5, baz: 3 }> {
                    <Block { foo: condition }>
                }
                <Block { foo: false }>
            }
        }
        <Block { baz: 69 }> {
            match (bar) {
                true => {
                    <Block { foo: true }>
                },
                false => {
                    <Block { foo: false }> {
                        <Block { foo: true }>
                        <Block { foo: false }>
                    }
                }
            }
        }
    }
}

#[test]
fn oof() {
    render(true);
}