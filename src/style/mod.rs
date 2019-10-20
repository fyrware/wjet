use crate::theme::Theme;

pub trait Style {
    fn paint() -> Theme;
}