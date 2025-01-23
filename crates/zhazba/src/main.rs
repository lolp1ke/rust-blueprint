mod args;


use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use clap::Parser;
use crossterm::terminal;

use mlua::{AnyUserData, Lua};
use zhazba_config::Config;
use zhazba_editor::Editor;

use args::Args;
use zhazba_models::{ActionUserDataFactory, KeyActionUserDataFactory};


fn main() -> anyhow::Result<()> {
  let args: Args = Args::parse();

  let work_folder: Option<PathBuf> = match args.work_folder {
    Some(path) => {
      let path: PathBuf = PathBuf::from(path);
      if !path.exists() {
        println!("Path does not exist");
        std::process::exit(1);
      };
      if !path.is_dir() {
        println!("Path must be directory");
        None
      } else {
        Some(path)
      }
    }

    None => None,
  };
  let config_source: String =
    match std::fs::read_to_string(PathBuf::from(".config/zhazba.config.lua")) {
      Ok(config) => config,
      Err(err) => panic!("{:?}", err),
    };


  let size: (u16, u16) = match terminal::size() {
    Ok(size) => size,
    Err(err) => panic!("{:?}", err),
  };


  let mut editor: Editor = Editor::new(work_folder.clone(), size);
  if let Some(_) = work_folder {
    editor.load_dir()?;
    // TODO: implement properly
    editor.current_buffer_idx = 0;
  };

  zhazba_lua::load(editor.config.clone(), &config_source).unwrap_or_else(|err| {
    panic!("FIXME:\n{:#?}", err);
  });

  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      editor.run().await?;

      return Ok::<(), anyhow::Error>(());
    })?;


  return Ok(());
}
