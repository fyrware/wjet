use crate::wml::wml_token;

#[macro_export]
macro_rules! wml_markup {

    (|$lexer: ident, $tree: ident| -> (<{ if $condition: expr }>$($wml: tt)*)) => {
        $lexer.start_condition($condition);

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.start_grammar(wml_token::Token::If);
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<{ else }>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => {
                $lexer.end_condition();
                $lexer.start_condition(true);
            },
            Some(true) => {
                $lexer.end_condition();
                $lexer.start_condition(false);
            },
            None => ()
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<;>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {

                match $lexer.grammar() {
                    Some(wml_token::Token::If) => {
                        $lexer.end_condition();
                    },
                    _ => ()
                }

                $lexer.end_grammar();
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<{ $expression: expr };>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                println!("EXPRESS: {}", stringify!($expression));
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<$type: ty { $($field: ident: $value: expr),* }>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                let mut widget = <$type>::new();

                $(widget.$field = $value;)*

                $lexer.start_grammar(wml_token::Token::NodeStart);
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<$type: ty { $($field: ident: $value: expr),* };>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.start_grammar(wml_token::Token::NodeStart);
                $lexer.end_grammar();
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<$type: ty;>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.end_grammar();
            }
        }

        wml_markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> ()) => {};
}