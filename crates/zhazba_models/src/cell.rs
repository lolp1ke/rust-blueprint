use crossterm::style::ContentStyle;

pub struct Cell {
  ch: char,
  style: ContentStyle,
}
