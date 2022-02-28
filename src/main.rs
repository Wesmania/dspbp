use args::Commands;
use blueprint::Blueprint;
use clap::StructOpt;
use data::{traits::{DSPEnum, Replace, ReplaceItem, ReplaceRecipe}, enums::{DSPItem, DSPRecipe}};
use error::some_error;
use strum::{ParseError, IntoEnumIterator};
use std::{io::{Read, Write, Seek, Cursor, Stdout}, collections::HashMap, fs::File};

use crate::stats::{Stats, GetStats};

mod args;
mod blueprint;
mod data;
mod error;
mod md5;
mod serialize;
mod stats;

fn iof(arg: &Option<String>) -> Option<&str> {
    match arg.as_ref().map(|x| x.as_ref()) {
        None | Some("-") => None,
        file => file,
    }
}

pub trait ReadPlusSeek: Read + Seek {}
impl<T: Read + Seek> ReadPlusSeek for T {}
pub trait WritePlusSeek: Write + Seek {}
impl<T: Write + Seek> WritePlusSeek for T {}

pub enum WriteSeek {
    File(File),
    BufOut(Cursor<Vec<u8>>, Stdout)
}

impl WriteSeek {
    fn flush_if_stdout(&mut self) -> std::io::Result<()> {
        if let Self::BufOut(c, s) = self {
            s.write_all(c.get_ref().as_ref())
        } else {
            Ok(())
        }
    }
}

impl Write for WriteSeek {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::File(f) => f.write(buf),
            Self::BufOut(c, _) => c.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::File(f) => f.flush(),
            Self::BufOut(c, _) => c.flush(),
        }
    }
}

impl Seek for WriteSeek {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            Self::File(f) => f.seek(pos),
            Self::BufOut(c, _) => c.seek(pos),
        }
    }
}

fn itob(i: &mut Box<dyn ReadPlusSeek>) -> anyhow::Result<Blueprint> {
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

    let mut input: Box<dyn ReadPlusSeek> = match iof(&args.input) {
        None => {
            let mut all_input = vec![];
            std::io::stdin().read_to_end(&mut all_input)?;
            Box::new(Cursor::new(all_input))
        },
        Some(file) => Box::new(std::fs::File::open(file)?),
    };
    let mut output: WriteSeek = match iof(&args.output) {
        None => WriteSeek::BufOut(Cursor::new(vec![]), std::io::stdout()),
        Some(file) => WriteSeek::File(
            std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(file)?,
        ),
    };

    match args.command {
        #[cfg(feature = "dump")]
        Commands::Dump => {
            let bp = itob(&mut input)?;
            output.write_all(&bp.dump_json()?)?;
            output.flush_if_stdout()?;
        }
        #[cfg(feature = "dump")]
        Commands::Undump => {
            let mut data = vec![];
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new_from_json(&data)?;
            output.write_all(bp.into_bp_string()?.as_bytes())?;
            output.flush_if_stdout()?;
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
            output.flush_if_stdout()?;
        }
        Commands::Info => {
            let bp = itob(&mut input)?;
            let mut stats = Stats::new();
            bp.get_stats(&mut stats);
            print!("{}", stats);
        }
        Commands::Items => {
            for e in DSPItem::iter() {
                println!("{}", e.as_ref())
            }
        }
        Commands::Recipes => {
            for e in DSPRecipe::iter() {
                println!("{}", e.as_ref())
            }
        }
    }
    Ok(())
}
