use crossterm::style::{Attribute, Color, SetForegroundColor};
use strum::IntoEnumIterator;

fn main() {
    let col_width = Color::iter()
        .take(Color::iter().count() - 2)
        .map(|c| format!("{c:?}").len())
        .max()
        .unwrap()
        + 2;

    for color in Color::iter()
        .take(Color::iter().count() - 2)
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
