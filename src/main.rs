use std::io::{Read, Write};
use args::Commands;
use blueprint::Blueprint;
use clap::StructOpt;

mod args;
mod blueprint;
mod error;
mod md5;
mod bpdata;
mod serialize;

fn iof(arg: &Option<String>) -> Option<&str> {
    match arg.as_ref().map(|x| x.as_ref()) {
        None | Some("-") => None,
        file => file,
    }
}

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut input: Box<dyn Read> = match iof(&args.input) {
        None => Box::new(std::io::stdin()),
        Some(file) => Box::new(std::fs::File::open(file)?),
    };
    let mut output: Box<dyn Write> = match iof(&args.output) {
        None => Box::new(std::io::stdout()),
        Some(file) => Box::new(std::fs::OpenOptions::new()
                          .write(true)
                          .truncate(true)
                          .create(true)
                          .open(file)?),
    };

    match args.command {
        Commands::Parse => {
            let mut data = vec!();
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new(&data)?;
            output.write_all(bp.into_bp_string()?.as_bytes())?;
        }
        Commands::Dump => {
            let mut data = vec!();
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new(&data)?;
            output.write_all(&bp.dump_json()?)?;
        }
        Commands::Undump => {
            let mut data = vec!();
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new_from_json(&data)?;
            output.write_all(bp.into_bp_string()?.as_bytes())?;
        }
    }
    Ok(())
}
