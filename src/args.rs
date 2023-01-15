use clap::{Parser, Subcommand};

/// Dyson Sphere Program blueprint tool.
///
/// For subcommand help, use 'dspbp help <subcommand>'.
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

#[derive(Parser, Debug)]
#[clap()]
pub struct EditArgs {
    /// Replace items with other items.
    /// Accepts format like this: "Item1:Replacement1,Item2:Replacement2,..."
    #[clap(short, long)]
    pub replace_item: Option<String>,
    /// Replace recipes with other recipes.
    /// Accepts format like this: "Recipe1:Replacement1,Recipe2:Replacement2,..."
    #[clap(short = 'R', long)]
    pub replace_recipe: Option<String>,
    /// Replace items with other items, also replacing their recipes.
    ///
    /// When there are multiple recipes available, chooses the most basic recipe.
    /// Replacements are overwritten by only-item and only-recipe replacements.
    /// Accepts format like this: "Item1:Replacement1,Item2:Replacement2,..."
    #[clap(short = 'B', long)]
    pub replace_both: Option<String>,
    /// Upgrade/downgrade buildings.
    ///
    /// Accepts format like this: "Building1:Replacement1,Building2:Replacement2,..."
    #[clap(short = 'b', long)]
    pub replace_building: Option<String>,
    /// Replace icon text.
    #[clap(short = 't', long)]
    pub icon_text: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Dump blueprint to JSON.
    #[cfg(feature = "dump")]
    Dump,
    /// Undump blueprint from JSON to blueprint format.
    #[cfg(feature = "dump")]
    Undump,
    /// Edit blueprint. Accepts more arguments.
    Edit(EditArgs),
    /// Print some blueprint info.
    Info,
    /// Print item names.
    Items,
    /// Print recipe names.
    Recipes,
}
