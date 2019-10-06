use crate::wml::wml_token;

#[macro_export]
macro_rules! wml_markup {
    // <Widget { field_1: value_1, field_2: value_2 }>;
    (|$tree: ident| -> (<$widget: ty { $($field: ident: $value: expr),* }>; $($next: tt)*)) => {
        $tree.branch();
        {
            let mut wml_widget = <$widget>::new(); $(wml_widget.$field = $value;)*
        }
        $tree.climb();
        wml_markup!(|$tree| -> ($($next)*));
    };
    // <Widget { field_1: value_1, field_2: value_2 }> { ... }
    (|$tree: ident| -> (<$widget: ty { $($field: ident: $value: expr),* }> { $($body: tt)* } $($next: tt)*)) => {
        $tree.branch();
        {
            let mut wml_widget = <$widget>::new(); $(wml_widget.$field = $value;)*

            wml_markup!(|$tree| -> ($($body)*));
        }
        $tree.climb();
        wml_markup!(|$tree| -> ($($next)*));
    };
    // for item in (iter) { ... }
    (|$tree: ident| -> (for $item: ident in ($iter: expr) { $($body: tt)* } $($next: tt)*)) => {
        for $item in $iter {
            wml_markup!(|$tree| -> ($($body)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // if (condition) { ... }
    (|$tree: ident| -> (if ($condition: expr) { $($body: tt)* } $($next: tt)*)) => {
        if $condition {
            wml_markup!(|$tree| -> ($($body)*));
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // match (value) { case_1 => { ... }, case_2 => { ... } }
    (|$tree: ident| -> (match ($value: expr) { $($case: expr => { $($body: tt)* }),* } $($next: tt)*)) => {
        match $value {
            $($case => { wml_markup!(|$tree| -> ($($body)*)); },)*
        }
        wml_markup!(|$tree| -> ($($next)*));
    };
    // end wml_markup
    (|$tree: ident| -> ()) => {};
}