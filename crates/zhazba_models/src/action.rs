use std::collections::HashMap;

use mlua::prelude::*;


pub struct UserDataFactory;
macro_rules! impl_lua_user_data {
  ($factory:ident, $ident:ident, { $($variant:ident($($args:ident: $types:ty),*)),* $(,)? }) => {
    pub struct $factory;

    impl LuaUserData for $factory {
      fn add_methods<M: LuaUserDataMethods<Self>>(#[warn(unused_variables)] methods: &mut M) {
        $(
          methods.add_method(stringify!($variant), |_, _, #[allow(unused_parens)] ($($args),*): ($($types),*)| {
            return Ok($ident::$variant($($args),*));
          });
        )*
      }
    }
    impl FromLua for $ident {
      fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        return Ok(match value.as_userdata() {
          Some(ud) => match ud.borrow::<Self>() {
            Ok(ud) => ud.clone(),

            _ => {
              return Err(LuaError::RuntimeError(
                format!("Expected {}", stringify!($ident)),
              ));
            }
          },

          _ => return Err(LuaError::RuntimeError("Expected userdata".to_string())),
        });
      }
    }
  };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
  Quit(bool),
  Save(),
  EnterMode(String),

  InsertCharAtCursor(String),

  Callback(String),
}
impl LuaUserData for Action {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &Self, ()| {
      return Ok(format!("{:?}", this));
    });
    methods.add_meta_method(LuaMetaMethod::Eq, |_, a: &Self, b: Self| {
      return Ok(*a == b);
    });
  }
}
impl_lua_user_data!(ActionUserDataFactory, Action, {
  Quit(force: bool),
  Save(),
  EnterMode(mode: String),

  InsertCharAtCursor(ch: String),

  Callback(name: String),
});


#[derive(Clone, Debug)]
pub enum KeyAction {
  Single(Action),
  Multiple(Vec<Action>),
  Nested(HashMap<String, KeyAction>),
}
impl LuaUserData for KeyAction {
  fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this: &Self, ()| {
      return Ok(format!("{:?}", this));
    });
  }
}
impl_lua_user_data!(KeyActionUserDataFactory, KeyAction, {
  Single(action: Action),
  Multiple(actions: Vec<Action>),
  Nested(map: HashMap<String, KeyAction>),
});
