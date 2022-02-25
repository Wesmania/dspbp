use clap::{Parser, Subcommand};

/// Dyson Sphere Program blueprint tool.
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
    /// Replace items with other items.
    /// Accepts format like this: "Item1:Replacement1,Item2:Replacement2,..."
    #[clap(short, long)]
    pub replace_item: Option<String>,
    /// Replace recipes with other recipes.
    /// Accepts format like this: "Recipe1:Replacement1,Recipe2:Replacement2,..."
    #[clap(short='R', long)]
    pub replace_recipe: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Dump blueprint to JSON.
    Dump,
    /// Undump blueprint from JSON to blueprint format.
    Undump,
    /// Edit blueprint.
    Edit,
    /// Print some blueprint info.
    Info,
}
