use std::io::Read;
use args::Commands;
use blueprint::Blueprint;
use clap::StructOpt;

mod args;
mod blueprint;
mod error;
mod md5;
mod bpdata;
mod serialize;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.command {
        Commands::Parse => {
            let mut stdin = std::io::stdin();
            let mut data = vec!();
            stdin.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new(&data)?;
            print!("{}", bp.into_bp_string()?);
        }
        Commands::Dump => {
            let mut stdin = std::io::stdin();
            let mut data = vec!();
            stdin.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new(&data)?;
            print!("{}", std::str::from_utf8(&bp.dump_json()?)?);
        }
        Commands::Undump => {
            let mut stdin = std::io::stdin();
            let mut data = vec!();
            stdin.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new_from_json(&data)?;
            print!("{}", bp.into_bp_string()?);
        }
    }
    Ok(())
}
