use crossterm::style::Color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

#[allow(dead_code)] // Originally this implemented the termion::color::Color Trait, will hopefully find a use soon
impl Type {
    pub fn to_color(self) -> Color {
        match self {
            Type::Number => Color::Rgb{r: 220, g: 163, b: 163},
            Type::Match => Color::Rgb{r: 38, g: 139, b: 210},
            Type::String => Color::Rgb{r: 211, g: 54, b: 130},
            Type::Character => Color::Rgb{r: 108, g: 113, b: 196},
            Type::Comment | Type::MultilineComment => Color::Rgb{r: 133, g: 153, b: 0},
            Type::PrimaryKeywords => Color::Rgb{r: 181, g: 137, b: 0},
            Type::SecondaryKeywords => Color::Rgb{r: 42, g: 161, b: 152},
            _ => Color::Rgb{r: 255, g: 255, b: 255},
        }
    }
}
