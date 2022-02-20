use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap()]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Dump,
}
