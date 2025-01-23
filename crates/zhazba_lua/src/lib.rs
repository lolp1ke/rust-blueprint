use mlua::prelude::*;
use mlua::AnyUserData;

use zhazba_config::Config;
use zhazba_models::ActionUserDataFactory;
use zhazba_models::KeyActionUserDataFactory;

pub fn load(config: Config, config_source: &String) -> LuaResult<()> {
  let lua: Lua = Lua::new();

  let config: AnyUserData = lua.create_userdata(config)?;
  lua.globals().set("Config", config)?;

  let action: AnyUserData = lua.create_userdata(ActionUserDataFactory)?;
  lua.globals().set("Action", action)?;

  let key_action: AnyUserData = lua.create_userdata(KeyActionUserDataFactory)?;
  lua.globals().set("KeyAction", key_action)?;


  lua.load(config_source).exec()?;
  return Ok(());
}
