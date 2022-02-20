use std::io::Read;
use args::Commands;
use blueprint::Blueprint;
use clap::StructOpt;

mod args;
mod blueprint;
mod md5;


fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.command {
        Commands::Dump => {
            let mut stdin = std::io::stdin();
            let mut data = vec!();
            stdin.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new(&data)?;
            print!("{}", bp.into_bp_string());
        }
    }
    Ok(())
}
