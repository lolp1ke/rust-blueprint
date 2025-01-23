use mlua::prelude::*;
use zhazba_models::KeyAction;

use crate::Config;


impl LuaUserData for Config {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_method(
      "keymap",
      |_, this: &Self, (key, mode, key_action): (String, String, KeyAction)| {
        return Ok(this.add_keymap(
          (key, mode.chars().next().unwrap_or_else(|| '\0')),
          key_action,
        ));
      },
    );
    methods.add_method("add_insert_buffer_mode", |_, this: &Self, mode: String| {
      let mode: char = mode.chars().next().unwrap_or_else(|| '\0');
      return Ok(this.add_insert_buffer_mode(mode));
    });


    methods.add_method("set_theme", |_, this: &Self, theme: String| {
      let old_theme: String = std::mem::replace(&mut this.borrow_mut().theme, theme);
      return Ok(old_theme);
    });
    methods.add_method("get_theme", |_, this: &Self, ()| {
      return Ok(this.borrow().theme.clone());
    });
  }
}
