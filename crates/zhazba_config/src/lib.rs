mod lua;


use std::{
  cell::RefCell,
  collections::HashMap,
  ops::{Deref, DerefMut},
  rc::Rc,
};


use zhazba_models::KeyAction;


#[derive(Clone, Debug, Default)]
pub struct ConfigInner {
  pub theme: String,

  pub keymaps: HashMap<(String, char), KeyAction>,
  pub insert_buffer_modes: Vec<char>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
  inner: Rc<RefCell<ConfigInner>>,
}
impl Deref for Config {
  type Target = Rc<RefCell<ConfigInner>>;

  fn deref(&self) -> &Self::Target {
    return &self.inner;
  }
}
impl DerefMut for Config {
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.inner;
  }
}


impl Config {
  pub fn add_keymap(&self, key: (String, char), key_action: KeyAction) -> Option<KeyAction> {
    let (key, mode): (String, char) = key;
    return self
      .borrow_mut()
      .keymaps
      .insert((key.to_lowercase(), mode), key_action);
  }

  pub fn add_insert_buffer_mode(&self, mode: char) {
    self.borrow_mut().insert_buffer_modes.push(mode);
  }
}
