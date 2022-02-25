use args::Commands;
use blueprint::Blueprint;
use clap::StructOpt;
use data::{traits::{DSPEnum, Replace, ReplaceItem, ReplaceRecipe}, enums::{DSPItem, DSPRecipe}};
use error::some_error;
use strum::ParseError;
use std::{io::{Read, Write}, collections::HashMap};

mod args;
mod blueprint;
mod data;
mod error;
mod md5;
mod serialize;

fn iof(arg: &Option<String>) -> Option<&str> {
    match arg.as_ref().map(|x| x.as_ref()) {
        None | Some("-") => None,
        file => file,
    }
}

fn itob(i: &mut Box<dyn Read>) -> anyhow::Result<Blueprint> {
    let mut data = vec![];
    i.read_to_end(&mut data)?;
    let data = String::from_utf8(data)?;
    Blueprint::new(&data)
}

fn parse_comma_list(s: &str) -> anyhow::Result<Vec<(String, String)>> {
    s.split(",").map(|v| {
        let p: Vec<&str> = v.split(":").collect();
        if p.len() != 2 {
            return Err(some_error("Invalid replacement format"));
        }
        Ok((p[0].to_owned(), p[1].to_owned()))
    }).collect()
}

fn parse_into_enum_map<T: DSPEnum + 'static>(s: &str) -> anyhow::Result<Box<Replace<T>>>
{
    let l = parse_comma_list(s)?;
    let mut map = HashMap::new();

    l.iter().try_for_each::<_, Result<(), ParseError>>(|(e1, e2)| {
        let e1 = T::try_from(e1.as_ref())?;
        let e2 = T::try_from(e2.as_ref())?;
        map.insert(e1, e2);
        Ok(())
    })?;
    Ok(Box::new(move |from| *map.get(&from).unwrap_or(&from)))
}

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut input: Box<dyn Read> = match iof(&args.input) {
        None => Box::new(std::io::stdin()),
        Some(file) => Box::new(std::fs::File::open(file)?),
    };
    let mut output: Box<dyn Write> = match iof(&args.output) {
        None => Box::new(std::io::stdout()),
        Some(file) => Box::new(
            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(file)?,
        ),
    };

    match args.command {
        Commands::Parse => {
            let bp = itob(&mut input)?;
            output.write_all(bp.into_bp_string()?.as_bytes())?;
        }
        Commands::Dump => {
            let bp = itob(&mut input)?;
            output.write_all(&bp.dump_json()?)?;
        }
        Commands::Undump => {
            let mut data = vec![];
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new_from_json(&data)?;
            output.write_all(bp.into_bp_string()?.as_bytes())?;
        }
        Commands::Edit => {
            let mut bp = itob(&mut input)?;
            if let Some(i) = args.replace_item {
                let replace = parse_into_enum_map::<DSPItem>(&i)?;
                bp.replace_item(&replace);
            }
            if let Some(i) = args.replace_recipe {
                let replace = parse_into_enum_map::<DSPRecipe>(&i)?;
                bp.replace_recipe(&replace);
            }
            output.write_all(bp.into_bp_string()?.as_bytes())?;
        }
    }
    Ok(())
}
