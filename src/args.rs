use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap()]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
    /// Input file. If absent or '-', reads standard input.
    #[clap(short, long)]
    pub input: Option<String>,
    /// Output file. If absent or '-', writes to standard output.
    #[clap(short, long)]
    pub output: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Dump,
    Parse,
    Undump,
}
