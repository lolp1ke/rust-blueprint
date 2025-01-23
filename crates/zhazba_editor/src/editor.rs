use std::{
  fs::DirEntry,
  io::{stdout, Stdout},
  path::PathBuf,
};

use crossterm::{
  cursor,
  event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  terminal, ExecutableCommand,
};
use futures::{future::FutureExt, select, StreamExt};

use zhazba_buffer::Buffer;
use zhazba_config::Config;
use zhazba_models::{Action, KeyAction};
use zhazba_theme::Theme;


pub struct Editor {
  theme: Theme,
  pub config: Config,

  work_folder: Option<PathBuf>,
  buffers: Vec<Buffer>,
  pub current_buffer_idx: usize,

  mode: char,
  size: (u16, u16),
  stdout: Stdout,

  pos: (usize, usize),
  v_pos: (usize, usize),
}
impl Editor {
  pub fn new(work_folder: Option<PathBuf>, size: (u16, u16)) -> Self {
    terminal::SetTitle("zhazba");


    return Self {
      theme: Theme::default(),
      config: Config::default(),

      work_folder,
      buffers: Vec::new(),
      current_buffer_idx: usize::MAX,

      mode: 'n',
      size,
      stdout: stdout(),

      pos: (0, 0),
      v_pos: (0, 0),
    };
  }
  pub fn load_dir(&mut self) -> anyhow::Result<()> {
    let work_folder: &PathBuf = self.work_folder.as_ref().expect("empty workfolder");
    visit_dirs(work_folder, &mut |dir_entry: &DirEntry| {
      let buffer: Buffer = Buffer::load_from_file(dir_entry.path());
      self.buffers.push(buffer);
    })?;
    println!("{:#?}", self.buffers);


    return Ok(());


    fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&DirEntry)) -> anyhow::Result<()> {
      if !dir.is_dir() {
        return Ok(());
      };
      for entry in std::fs::read_dir(dir)? {
        let entry: DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
          visit_dirs(&path, cb)?;
        } else {
          cb(&entry);
        };
      }


      return Ok(());
    }
  }

  pub async fn run(&mut self) -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    self
      .stdout
      .execute(terminal::EnterAlternateScreen)?
      .execute(terminal::Clear(terminal::ClearType::All))?
      .execute(cursor::MoveTo(0, 0))?;

    let mut event_stream: EventStream = EventStream::new();
    loop {
      let mut delay = futures_timer::Delay::new(std::time::Duration::from_millis(10)).fuse();
      let mut event = event_stream.next().fuse();

      select! {
        _ = delay => {},

        event = event => {
          match &event {
            Some(Ok(ev)) => {
              if let Some(key_action) = self.handle_event(&ev) {
                if self.handle_key_action(key_action)? {
                  break;
                };
              };
            },

            Some(Err(err)) => panic!("{:?}", err),
            None => {},
          }
        }
      }
    }


    terminal::disable_raw_mode()?;
    self.stdout.execute(terminal::LeaveAlternateScreen)?;
    return Ok(());
  }
  fn handle_event(&self, ev: &Event) -> Option<KeyAction> {
    match ev {
      &Event::Key(KeyEvent {
        code,
        modifiers,
        kind,
        state: _,
      }) if kind == KeyEventKind::Press => {
        let key: String = format!(
          "{}{}{}",
          modifiers,
          if modifiers.is_empty() { "" } else { "-" },
          code
        )
        .to_lowercase();

        if self
          .config
          .borrow()
          .insert_buffer_modes
          .contains(&self.mode)
        {
          let ch: Option<char> = match ev {
            &Event::Key(KeyEvent { code, .. }) => match code {
              KeyCode::Char(ch) => Some(ch),

              _ => None,
            },

            _ => None,
          };
          if let Some(ch) = ch {
            return Some(KeyAction::Single(Action::InsertCharAtCursor(
              ch.to_string(),
            )));
          }
        }

        return self.config.borrow().keymaps.get(&(key, self.mode)).cloned();
      }

      _ => return None,
    };
  }
  fn handle_key_action(&mut self, key_action: KeyAction) -> anyhow::Result<bool> {
    return match key_action {
      KeyAction::Single(action) => self.execute_action(action),

      ka => todo!("{:?}", ka),
    };
  }
  fn execute_action(&mut self, action: Action) -> anyhow::Result<bool> {
    use Action::*;

    match action {
      Quit(force) => {
        if force {
          return Ok(true);
        };
        for buffer in self.buffers.iter() {
          if buffer.is_dirty() {
            return Ok(true);
          };
        }

        // TODO: create alert window
        println!("Unsaved file");
      }
      EnterMode(mode) => {
        let mode: char = mode.chars().next().unwrap_or_else(|| '\0');
        self.mode = mode;
      }
      InsertCharAtCursor(ch) => {
        let ch: char = ch.chars().next().unwrap_or_else(|| '\0');
        let pos: (usize, usize) = (self.pos.0, self.buffer_line());
        self.get_buffer_mut().insert(pos, ch);
        self.pos.0 += 1;
      }
      Save() => self.get_buffer_mut().save()?,

      action => todo!("{:?}", action),
    };

    return Ok(false);
  }


  fn buffer_line(&self) -> usize {
    return self.pos.1.saturating_add(self.v_pos.1);
  }


  fn get_buffer_mut(&mut self) -> &mut Buffer {
    return &mut self.buffers[self.current_buffer_idx];
  }
}
