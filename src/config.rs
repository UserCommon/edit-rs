use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

/// Configuration struct
#[derive(Clone)]
pub struct Config {
    pub padding_size: u16,
    pub header_height: u16,
    pub footer_height: u16,
    pub default_font_color: Color,
    pub default_background_color: Color,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn rgb_fg(color: Color) -> SetForegroundColor {
        // Get the text ANSI code from an RGB value
        SetForegroundColor(color)
    }
    pub fn rgb_bg(color: Color) -> SetBackgroundColor {
        // Get the background ANSI code from an RGB value
        SetBackgroundColor(color)
    }


}

pub struct ConfigBuilder {
    padding_size: u16,
    header_height: u16,
    footer_height: u16,
    default_font_color: Color,
    default_background_color: Color
}

impl ConfigBuilder {
    pub fn build(&mut self) -> Config {
        Config {
            padding_size: self.padding_size,
            header_height: self.header_height,
            footer_height: self.footer_height,
            default_font_color: self.default_font_color,
            default_background_color: self.default_background_color,
        }
    }

    pub fn padding_size(&mut self, size: u16) -> &mut Self {
        self.padding_size = size;
        self
    }

    pub fn header_height(&mut self, size: u16) -> &mut Self {
        self.header_height = size;
        self
    }

    pub fn footer_height(&mut self, size: u16) -> &mut Self {
        self.footer_height = size;
        self
    }

    pub fn default_font_color(&mut self, color: Color) -> &mut Self {
        self.default_font_color = color;
        self
    }

    pub fn default_background_color(&mut self, color: Color) -> &mut Self {
        self.default_background_color = color;
        self
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        let padding_size = 4;
        let header_height = 1;
        let footer_height = 1;
        let default_font_color = Color::Rgb {r: 255, g: 255, b: 255};
        let default_background_color = Color::Rgb {r: 44, g: 44, b: 44};

        ConfigBuilder {
            padding_size,
            header_height,
            footer_height,
            default_font_color,
            default_background_color
        }
    }
}