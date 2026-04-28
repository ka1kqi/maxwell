use cellophane::crossterm::style::Color;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum CatColor {
    /// Soft lavender (default)
    Lavender,
    /// Soft pink
    Pink,
    /// Mint green
    Mint,
    /// Warm peach
    Peach,
    /// Sky blue
    Sky,
    /// Cream
    Cream,
    /// Bright white
    White,
    /// Bright yellow
    Yellow,
    /// Use the terminal's default text color (best on unusual backgrounds)
    None,
}

impl CatColor {
    pub fn to_color(self) -> Option<Color> {
        match self {
            CatColor::Lavender => Some(Color::Rgb {
                r: 177,
                g: 156,
                b: 217,
            }),
            CatColor::Pink => Some(Color::Rgb {
                r: 255,
                g: 182,
                b: 193,
            }),
            CatColor::Mint => Some(Color::Rgb {
                r: 152,
                g: 230,
                b: 200,
            }),
            CatColor::Peach => Some(Color::Rgb {
                r: 255,
                g: 200,
                b: 160,
            }),
            CatColor::Sky => Some(Color::Rgb {
                r: 152,
                g: 200,
                b: 240,
            }),
            CatColor::Cream => Some(Color::Rgb {
                r: 255,
                g: 236,
                b: 200,
            }),
            CatColor::White => Some(Color::White),
            CatColor::Yellow => Some(Color::Yellow),
            CatColor::None => None,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "An animated cat for your terminal (Maxwell, courtesy of ascii-live)", long_about = None)]
pub struct Cli {
    /// Color of the cat. Use `none` to inherit your terminal's default text color
    /// (helpful if you have an unusual background).
    #[arg(long, value_enum, default_value_t = CatColor::Lavender)]
    pub color: CatColor,

    /// Background color tint for the whole screen while maxwell runs.
    /// Default `none` keeps your terminal's normal background.
    #[arg(long, value_enum, default_value_t = CatColor::None)]
    pub bg: CatColor,
}
