#[derive(clap::Parser, Debug)]
pub(crate) struct Args {
  pub work_folder: Option<String>,

  #[clap(long = "config", short = 'c')]
  pub config: Option<String>,
}
