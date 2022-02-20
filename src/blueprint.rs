use std::str::FromStr;
use std::io::{Read, Write};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use thiserror::Error;

pub struct Blueprint {
    layout: u32,
    icons: [u32; 5],
    timestamp: u64,
    game_version: String,
    short_desc: String,
    data: Vec<u8>,
    data_hash: String,
}

#[derive(Debug, Error)]
pub enum BpError {
    #[error("{0}")]
    E(String),
}

impl<T: Into<String>> From<T> for BpError {
    fn from(s: T) -> Self {
        Self::E(s.into())
    }
}

fn some_error<T: Into<String>>(s: T) -> anyhow::Error {
    BpError::from(s).into()
}

impl Blueprint {
    fn int<T: FromStr>(data: &str, what: &str) -> Result<T, BpError> {
        str::parse(data).map_err(|_| format!("Failed to parse {}", what).into())
    }

    fn unpack_data(b64data_hash: &str) -> anyhow::Result<(Vec<u8>, String)> {
        let [empty, b64data, hash]: [&str; 3] = b64data_hash.split('"').collect::<Vec<&str>>().try_into().unwrap();
        if !empty.is_empty() {
            return Err(some_error("Unexpected data before quoted blueprint"));
        }
        let zipped_data = base64::decode(b64data).map_err(|_| some_error("Failed to base64 decode blueprint"))?;
        let mut d = GzDecoder::new(zipped_data.as_slice());
        let mut data = vec![];
        d.read_to_end(&mut data)?;
        Ok((data, hash.to_owned()))
    }

    fn pack_data(&self) -> String {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(&self.data).unwrap();
        let gzipped_data = e.finish().unwrap();
        let b64_data = base64::encode(gzipped_data.as_slice());
        format!("\"{}\"{}", b64_data, self.data_hash)
    }

    pub fn new(mut data: &str) -> anyhow::Result<Self> {
        const PREFIX: &str = "BLUEPRINT:";
        if data.len() < PREFIX.len() || &data[0..PREFIX.len()] != PREFIX {
            let ml = std::cmp::min(PREFIX.len(), data.len());
            return Err(some_error(format!("Unexpected prefix: {}", &data[0..ml])))
        }
        data = &data[PREFIX.len()..];

        let fields: Vec<&str> = data.split(',').collect();
        if fields.len() != 12 {
            return Err(some_error(format!("Expected 12 CSV elements, got {}", fields.len())));
        }

        let [fixed0_1, layout]: [&str; 2] = fields[0..2].try_into().unwrap();
        let icons = &fields[2..7];
        let [fixed0_2, timestamp, game_version, short_desc, b64data_hash]: [&str; 5] = fields[7..12].try_into().unwrap();

        let fixed0_1: u32 = Self::int(fixed0_1, "fixed0_1")?;
        let layout = Self::int(layout, "layout")?;
        let icons: Vec<u32> = icons.into_iter().map(|x| Self::int(*x, "icon")).collect::<Result<Vec<_>, _>>()?;
        let fixed0_2: u32 = Self::int(fixed0_2, "fixed0_2")?;
        let timestamp = Self::int(timestamp, "timestamp")?;

        if fixed0_1 != 0 {
            return Err(some_error("fixed0_1 is not 0"));
        }
        if fixed0_2 != 0 {
            return Err(some_error("fixed0_2 is not 0"));
        }

        let (data, data_hash) = Self::unpack_data(b64data_hash)?;

        Ok(Self {
            layout,
            icons: icons.try_into().unwrap(),
            timestamp,
            game_version: game_version.into(),
            short_desc: short_desc.into(),
            data,
            data_hash,
        })
    }

    pub fn into_bp_string(&self) -> String {
        let icons = self.icons.map(|x| x.to_string()).join(",");
        format!("BLUEPRINT:0,{},{},0,{},{},{},{}",
                self.layout, icons, self.timestamp, self.game_version,
                self.short_desc, self.pack_data())
    }
}
