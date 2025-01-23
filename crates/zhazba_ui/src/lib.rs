use mlua::prelude::*;
use ratatui::prelude::*;


trait Drawable {
  fn draw(frame: &mut Frame);
}
pub struct Label {
  pos: (usize, usize),
  text: Vec<char>,
}

impl Drawable for Label {
  fn draw(frame: &mut Frame) {
    todo!()
  }
}

enum Element {
  Label { text: String },
  Button { label: String, cb: LuaFunction },
  Group { elements: Vec<Element> },
}
