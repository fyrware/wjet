#[macro_export]
macro_rules! markup {

    (|$lexer: ident, $tree: ident| -> (<{ if $condition: expr }>$($wml: tt)*)) => {
        $lexer.start_condition($condition);

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.start_grammar(crate::wml::wml_token::Token::If);
            }
        }

        markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<;>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {

                match $lexer.grammar() {
                    Some(crate::wml::wml_token::Token::If) => {
                        $lexer.end_condition();
                    },
                    _ => ()
                }

                $lexer.end_grammar();
            }
        }

        markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<$type: ty { $($key: ident: $value: expr),* }>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.start_grammar(crate::wml::wml_token::Token::NodeStart);
            }
        }

        markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<$type: ty { $($key: ident: $value: expr),* };>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.start_grammar(crate::wml::wml_token::Token::NodeStart);
                $lexer.end_grammar();
            }
        }

        markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> (<;$type: ty>$($wml: tt)*)) => {

        match $lexer.condition() {
            Some(false) => (),
            Some(true) | None => {
                $lexer.end_grammar();
            }
        }

        markup!(|$lexer, $tree| -> ($($wml)*));
    };

    (|$lexer: ident, $tree: ident| -> ()) => {};
}