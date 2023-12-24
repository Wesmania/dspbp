use args::Commands;
use blueprint::Blueprint;
use clap::Parser;
use data::{
    enums::{DSPItem, DSPRecipe},
    traits::{DSPEnum, TryFromUserString},
};
use edit::EditBlueprint;
use error::some_error;
use locale::{Locale, GLOBAL_SERIALIZATION_LOCALE};
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Seek, Stdout, Write},
};
use strum::IntoEnumIterator;

use crate::{data::visit::Visitor, edit::stats::GetStats};

pub(crate) mod args;
pub(crate) mod blueprint;
pub(crate) mod data;
pub(crate) mod edit;
pub(crate) mod error;
pub(crate) mod locale;
pub(crate) mod md5;
#[cfg(feature = "python")]
pub(crate) mod python;
pub(crate) mod stats;
#[cfg(test)]
pub(crate) mod testutil;

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
    BufOut(Cursor<Vec<u8>>, Stdout),
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
    s.split(",")
        .map(|v| {
            let p: Vec<&str> = v.split(":").collect();
            if p.len() != 2 {
                return Err(some_error(format!("Invalid input in replacement list: \"{}\". Expected two names separated by a colon, ':'.", v)));
            }
            Ok((p[0].to_owned(), p[1].to_owned()))
        })
        .collect()
}

fn parse_into_enum_map<T: DSPEnum + 'static>(s: &str) -> anyhow::Result<HashMap<T, T>> {
    let l = parse_comma_list(s)?;
    let mut map = HashMap::new();

    l.iter()
        .try_for_each::<_, Result<(), anyhow::Error>>(|(e1, e2)| {
            let e1 = T::try_from_user_string(e1.as_ref())?;
            let e2 = T::try_from_user_string(e2.as_ref())?;
            map.insert(e1, e2);
            Ok(())
        })?;
    Ok(map)
}

pub fn cmdline() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let input = || -> anyhow::Result<Box<dyn ReadPlusSeek>> {
        match iof(&args.input) {
            None => {
                let mut all_input = vec![];
                eprintln!("Reading blueprint from standard input.");
                std::io::stdin().read_to_end(&mut all_input)?;
                Ok(Box::new(Cursor::new(all_input)))
            }
            Some(file) => Ok(Box::new(std::fs::File::open(file)?)),
        }
    };
    let output = || -> anyhow::Result<WriteSeek> {
        match iof(&args.output) {
            None => Ok(WriteSeek::BufOut(Cursor::new(vec![]), std::io::stdout())),
            Some(file) => Ok(WriteSeek::File(
                std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(file)?,
            )),
        }
    };

    match args.command {
        #[cfg(feature = "dump")]
        Commands::Dump(args) => {
            if args.human_readable {
                let locale = match &args.locale {
                    None => Locale::en,
                    Some(s) => Locale::try_from_user_string(s)?,
                };
                let _ = GLOBAL_SERIALIZATION_LOCALE.set(locale);
            }
            let mut input = input()?;
            let mut output = output()?;
            let bp = itob(&mut input)?;
            output.write_all(&bp.dump_json()?)?;
            output.flush_if_stdout()?;
        }
        #[cfg(feature = "dump")]
        Commands::Undump => {
            let mut data = vec![];
            let mut input = input()?;
            let mut output = output()?;
            input.read_to_end(&mut data)?;
            let data = String::from_utf8(data)?;
            let bp = Blueprint::new_from_json(&data)?;
            output.write_all(bp.into_bp_string(args.compression_level)?.as_bytes())?;
            output.flush_if_stdout()?;
        }
        Commands::Edit(eargs) => {
            let mut input = input()?;
            let mut output = output()?;
            let mut bp = EditBlueprint::new(itob(&mut input)?);

            let mut item_replace = HashMap::new();
            let mut recipe_replace = HashMap::new();
            let mut building_replace = HashMap::new();

            // This goes first so it can be overwritten.
            if let Some(i) = eargs.replace_both {
                let mut r = parse_into_enum_map::<DSPItem>(&i)?;
                let mut r2: HashMap<DSPRecipe, DSPRecipe> = r
                    .iter()
                    .filter_map(|(k, v)| {
                        let k = DSPRecipe::for_item(k)?;
                        let v = DSPRecipe::for_item(v)?;
                        Some((k, v))
                    })
                    .collect();
                item_replace.extend(r.drain());
                recipe_replace.extend(r2.drain());
            }
            if let Some(i) = eargs.replace_item {
                let mut r = parse_into_enum_map::<DSPItem>(&i)?;
                item_replace.extend(r.drain());
            }
            if let Some(i) = eargs.replace_recipe {
                let mut r = parse_into_enum_map::<DSPRecipe>(&i)?;
                recipe_replace.extend(r.drain());
            }

            if let Some(i) = eargs.replace_building {
                building_replace = parse_into_enum_map::<DSPItem>(&i)?;
            }

            if !item_replace.is_empty() {
                bp.replace_item(item_replace);
            }
            if !recipe_replace.is_empty() {
                bp.replace_recipe(recipe_replace);
            }

            if !building_replace.is_empty() {
                bp.replace_building(building_replace)?;
            }

            if let Some(i) = eargs.icon_text {
                bp.set_icon_text(&i);
            }

            if eargs.round {
                bp.round_floats();
            }

            output.write_all(bp.0.into_bp_string(args.compression_level)?.as_bytes())?;
            output.flush_if_stdout()?;
        }
        Commands::Info => {
            let mut input = input()?;
            let mut bp = itob(&mut input)?;
            println!("{}", bp.get_description()?);
            let mut stats = GetStats::new();
            stats.visit_blueprint(&mut bp);
            print!("{}", stats.0);
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
