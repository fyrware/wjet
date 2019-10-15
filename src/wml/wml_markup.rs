use crate::wml::wml_token;

#[macro_export]
macro_rules! wml_markup {
    // 0) <Widget> { ... } -or- <Widget { field_1: value_1, field_2: value_2 }> { ... }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (<$widget: ty $({ $($field: ident: $value: expr),* })*> { $($body: tt)* } $($next: tt)*)) => {
        $tree.branch();
        {
            let mut wjet_parent = &$parent;
            let mut wjet_widget = <$widget>::new(); $($(wjet_widget.$field = $value;)*)*
            wml_markup!(|$tree, $lexer, $parent| -> ($($body)*));
        }
        $tree.climb();
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 1) <Widget> -or- <Widget { field_1: value_1, field_2: value_2 }>
    (|$tree: ident, $lexer: ident, $parent: ident| -> (<$widget: ty $({ $($field: ident: $value: expr),* })*> $($next: tt)*)) => {
        $tree.branch();
        {
            let mut wjet_parent = &$parent;
            let mut wjet_widget = <$widget>::new(); $($(wjet_widget.$field = $value;)*)*
        }
        $tree.climb();
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 2) for item in (iter) { ... }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (for $item: ident in ($iter: expr) { $($body: tt)* } $($next: tt)*)) => {
        for $item in $iter {
            wml_markup!(|$tree, $lexer, $parent| -> ($($body)*));
        }
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 3) if (condition) { ... } else { ... } -or-  if (condition) { ... } else if (branch) { ... } else { ... }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (if ($condition: expr) { $($body: tt)* } $(else if ($branch: expr) { $($branch_body: tt)* })* else { $($else: tt)* } $($next: tt)*)) => {
        if $condition {
            wml_markup!(|$tree, $lexer, $parent| -> ($($body)*));
        } $(else if $branch { wml_markup!(|$tree, $lexer, $parent| -> ($($branch_body)*)); })* else {
            wml_markup!(|$tree, $lexer, $parent| -> ($($else)*));
        }
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 4) if (condition) { ... }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (if ($condition: expr) { $($body: tt)* } $($next: tt)*)) => {
        if $condition {
            wml_markup!(|$tree, $lexer, $parent| -> ($($body)*));
        }
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 5) match (value) { case_1 => { ... }, case_2 => { ... } }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (match ($value: expr) { $($case: expr => { $($body: tt)* }),* } $($next: tt)*)) => {
        match $value {
            $($case => { wml_markup!(|$tree, $lexer, $parent| -> ($($body)*)); },)*
        }
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 6) while (condition) { ... }
    (|$tree: ident, $lexer: ident, $parent: ident| -> (while ($condition: expr) { $($body: tt)* } $($next: tt)*)) => {
        while $condition {
            wml_markup!(|$tree, $lexer, $parent| -> ($($body)*));
        }
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 7) statement;
    (|$tree: ident, $lexer: ident, $parent: ident| -> ($statement: stmt; $($next: tt)*)) => {
        $statement;
        wml_markup!(|$tree, $lexer, $parent| -> ($($next)*));
    };
    // 8) end wml_markup
    (|$tree: ident, $lexer: ident, $parent: ident| -> ()) => {};
}