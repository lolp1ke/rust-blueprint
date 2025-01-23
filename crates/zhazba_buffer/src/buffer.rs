use std::path::PathBuf;

#[derive(Debug)]
pub struct Buffer {
  pub file: PathBuf,
  changed: bool,

  pub source: String,
  pub lines: Vec<String>,

  pub pos: (usize, usize),
  pub v_offset: usize,
}
impl Buffer {
  pub fn load_from_file(file: PathBuf) -> Self {
    let mut source: String = match std::fs::read_to_string(&file) {
      Ok(source) => source,
      Err(err) => panic!("{:?}", err),
    };
    source.push('\n');
    let lines: Vec<String> = source.lines().map(|s: &str| s.to_string()).collect();


    return Self {
      file,
      changed: false,

      source,
      lines,

      pos: (0, 0),
      v_offset: 0,
    };
  }
  pub fn save(&mut self) -> anyhow::Result<()> {
    let file: &PathBuf = &self.file;
    let mut contents: String = self.contents();
    contents.push('\n');
    std::fs::write(file, contents)?;
    self.changed = false;


    return Ok(());
  }


  pub fn contents(&self) -> String {
    return self.lines.join(&"\n");
  }

  pub fn insert(&mut self, (x, y): (usize, usize), ch: char) {
    self.lines[y].insert(x, ch);
    self.changed = true;
  }


  pub fn is_dirty(&self) -> bool {
    return !self.changed;
  }
}
