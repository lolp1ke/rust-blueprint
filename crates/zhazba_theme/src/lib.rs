use crossterm::style::Color;

#[derive(Default)]
pub struct Theme {
  pub name: String,
  pub style: Style,
}


pub struct Style {
  fg: Color,
  bg: Color,

  bold: bool,
  italic: bool,
}
impl Default for Style {
  fn default() -> Self {
    return Self {
      fg: Color::White,
      bg: Color::Black,

      bold: false,
      italic: false,
    };
  }
}
