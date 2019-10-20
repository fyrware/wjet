use crate::tree::Tree;
use crate::widget::Widget;
use std::rc::Rc;

macro_rules! wml_markup {
    // 0) <Widget[ name ]> { ... } -or- <Widget[ name ] { field_1: value_1, field_2: value_2 }> { ... }
    (|$tree: ident| -> (<$widget: ty $([ $name: expr ])* $({ $($field: ident: $value: expr),* })*> { $($body: tt)* } $($next: tt)*)) => {
        {
            let mut wml_widget = <$widget>::new(); $($(wml_widget.$field = $value;)*)*
            let mut wml_widget_name = "".to_string() $(+ &$name.to_string())*;
            $tree.branch(Rc::new(wml_widget), wml_widget_name);
            wml_markup!(|$tree| -> ($($body)*));
            $tree.climb();
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // 1) <Widget[ name ]> -or- <Widget[ name ] { field_1: value_1, field_2: value_2 }>
    (|$tree: ident| -> (<$widget: ty $([ $name: expr ])* $({ $($field: ident: $value: expr),* })*> $($next: tt)*)) => {
        {
            let mut wml_widget = <$widget>::new(); $($(wml_widget.$field = $value;)*)*
            let mut wml_widget_name = "".to_string() $(+ &$name.to_string())*;
            $tree.branch(Rc::new(wml_widget), wml_widget_name);
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

#[test]
fn noop() {}