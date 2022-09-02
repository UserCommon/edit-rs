use crossterm::style::{Color};

/// Configuration struct
pub struct Config {
    pub padding_size: u16,
    pub header_height: u16,
    pub footer_height: u16,
    pub default_font_color: Color,
    pub default_background_color: Color,
}

impl Config {
    pub fn new(padding: u16, header_s: u16, footer_s: u16, default_fg: Color, default_bg: Color) -> Self {
        Config {
            padding_size: padding,
            header_height: header_s,
            footer_height: footer_s,
            default_font_color: default_fg,
            default_background_color: default_bg,
        }
    }
}