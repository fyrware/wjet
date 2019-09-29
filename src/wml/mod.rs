#[macro_use]
mod wml_markup;

pub mod wml_lexer;
pub mod wml_token;

#[macro_export]
macro_rules! wml {

    ($($wml:tt)*) => {
        {
            let mut lexer = wml_lexer::Lexer::new();
            let mut tree = crate::core::core_tree::Tree::new();

            markup!(|lexer, tree| -> ($($wml)*));
        }
    };
}

struct Block {}

impl  Block {
    fn new() -> Block {
        Block {}
    }
}

impl crate::core::core_widget::Widget for Block {
    fn render(&self) -> crate::core::core_tree::Tree {
        unimplemented!()
    }
}

#[test]
fn oof() {
    let condition = true;

    return wml!(
        <Block {}>
            <{ if condition }>
                <Block {};>
            <;>
        <;Block>
    );
}