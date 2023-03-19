use clap::{Parser};


#[derive(clap::Subcommand, Debug)]
pub enum Action {
   Run {
        version: String,
   },
   Remove,
}

#[derive(clap::Parser, Debug)]
pub struct LauncherArgs {
   #[command(subcommand)]
   pub action: Action,

   #[clap(default_value_t=4,short, long,help="The amount of threads to use when downloading files")]
   pub threads: u8,
   
   #[clap(default_value_t=(".minecraft").to_string(), short, long,help="Folder name of a game")]
   pub game_dir: String,
}
