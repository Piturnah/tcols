use crossterm::style::{Attribute, Color, SetForegroundColor};
use strum::IntoEnumIterator;

fn main() {
    let col_width = Color::iter()
        .enumerate()
        .filter(|(i, _)| i < &(Color::iter().count() - 2))
        .map(|(_, c)| format!("{c:?}").len())
        .max()
        .unwrap()
        + 2;

    for color in Color::iter()
        .enumerate()
        .filter(|(i, _)| i < &(Color::iter().count() - 2))
        .map(|(_, c)| c)
    {
        println!(
            "{}{: <col_width$}{}{:?}{}",
            SetForegroundColor(color),
            format!("{:?}", color),
            Attribute::Reverse,
            color,
            Attribute::Reset,
        );
    }
}
