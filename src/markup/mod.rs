use crate::tree::Tree;
use crate::widget::Widget;
use std::boxed::Box;

macro_rules! wml_markup {
    // 0) <Widget> { ... } -or- <Widget { field_1: value_1, field_2: value_2 }> { ... }
    (|$tree: ident| -> (<$widget: ty $({ $($field: ident: $value: expr),* })*> { $($body: tt)* } $($next: tt)*)) => {
        {
            let mut wml_widget = <$widget>::new(); $($(wml_widget.$field = $value;)*)*
            $tree.branch(Box::new(wml_widget));
            wml_markup!(|$tree| -> ($($body)*));
            $tree.climb();
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 1) <Widget> -or- <Widget { field_1: value_1, field_2: value_2 }>
    (|$tree: ident| -> (<$widget: ty $({ $($field: ident: $value: expr),* })*> $($next: tt)*)) => {
        {
            let mut wml_widget = <$widget>::new(); $($(wml_widget.$field = $value;)*)*
            $tree.branch(Box::new(wml_widget));
            $tree.climb();
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 2) for item in (iter) { ... }
    (|$tree: ident| -> (for $item: ident in ($iter: expr) { $($body: tt)* } $($next: tt)*)) => {
        for $item in $iter {
            wml_markup!(|$tree| -> ($($body)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 3) if (condition) { ... } else { ... } -or-  if (condition) { ... } else if (branch) { ... } else { ... }
    (|$tree: ident| -> (if ($condition: expr) { $($body: tt)* } $(else if ($branch: expr) { $($branch_body: tt)* })* else { $($else: tt)* } $($next: tt)*)) => {
        if $condition {
            wml_markup!(|$tree| -> ($($body)*));
        } $(else if $branch { wml_markup!(|$tree| -> ($($branch_body)*)); })* else {
            wml_markup!(|$tree| -> ($($else)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 4) if (condition) { ... }
    (|$tree: ident| -> (if ($condition: expr) { $($body: tt)* } $($next: tt)*)) => {
        if $condition {
            wml_markup!(|$tree| -> ($($body)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 5) match (value) { case_1 => { ... }, case_2 => { ... } }
    (|$tree: ident| -> (match ($value: expr) { $($case: expr => { $($body: tt)* }),* } $($next: tt)*)) => {
        match $value {
            $($case => { wml_markup!(|$tree| -> ($($body)*)); },)*
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 6) while (condition) { ... }
    (|$tree: ident| -> (while ($condition: expr) { $($body: tt)* } $($next: tt)*)) => {
        while $condition {
            wml_markup!(|$tree| -> ($($body)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 7) statement;
    (|$tree: ident| -> ($statement: stmt; $($next: tt)*)) => {
        $statement;
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 8) end wml_markup
    (|$tree: ident| -> ()) => {}
}

#[macro_export]
macro_rules! wml {

    ($($wml:tt)*) => {
        {
            let mut wml_tree = Tree::new();
            wml_markup!(|wml_tree| -> ($($wml)*));
            wml_tree
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

impl Widget for Block {

}

fn render(condition: bool) -> Tree {
    let bar = false;

    wml! {
        <Block> {
            <Block>
        }
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