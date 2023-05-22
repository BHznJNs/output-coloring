use std::fmt::Display;

use crate::{color::Color, style::Style};

pub struct ColoredTemplate {
    pub ft_color: Color,
    pub bg_color: Option<Color>,
    pub style: Style,
}

impl ColoredTemplate {
    pub fn output<T: Display>(&self, content: T) -> String {
        let ft_color_str = self.ft_color.get_fg_str();
        let style_str = self.style.get_str();

        let bg_color_str = match self.bg_color {
            Some(bg) => format!("\x1b[{}m", bg.get_bg_str()),
            None => "".to_owned(),
        };

        format!("\x1b[{style_str};{ft_color_str}m{bg_color_str}{content}\x1b[0m")
    }
}

// impl fmt::Display for ColoredString {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let text = &self.content;

//         let ft_color_str = self.ft_color.get_fg_str();
//         let bg_color_str = self.bg_color.get_bg_str();
//         let style_str = self.style.get_str();

//         write!(f, "\x1b[{style_str};{ft_color_str}m\x1b[{bg_color_str}m{text}\x1b[0m")
//     }
// }
