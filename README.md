# output-coloring

Coloring terminal output, using template.

## Sample Code

```rust
extern crate output_coloring;

use output_coloring::*;

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
```