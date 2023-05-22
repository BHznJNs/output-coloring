extern crate output_coloring;

use output_coloring::*;

/// Use templates!
/// {
///     ft_color: font-color
///     bg_color: background-color
///     style   : font-style
/// }
const TEST: ColoredTemplate = ColoredTemplate {
    ft_color: Color::White,
    bg_color: Some(Color::BrightWhite),
    style: Style::Bold,
};
const WARN_TEMPLATE: ColoredTemplate = ColoredTemplate {
    ft_color: Color::White,
    bg_color: Some(Color::Yellow),
    style: Style::Bold,
};

fn main() {
    println!("{}", TEST.output("Hello World!"));
    println!("{}", WARN_TEMPLATE.output(" WARNING! "));
}