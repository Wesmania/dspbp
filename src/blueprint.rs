use std::str::FromStr;

use thiserror::Error;

pub struct Blueprint {
    layout: u32,
    icons: [u32; 5],
    timestamp: u64,
    game_version: String,
    short_desc: String,
    b64data_hash: String,
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

impl Blueprint {
    fn int<T: FromStr>(data: &str, what: &str) -> Result<T, BpError> {
        str::parse(data).map_err(|_| format!("Failed to parse {}", what).into())
    }

    pub fn new(mut data: &str) -> Result<Self, BpError> {
        const PREFIX: &str = "BLUEPRINT:";
        if data.len() < PREFIX.len() || &data[0..PREFIX.len()] != PREFIX {
            let ml = std::cmp::min(PREFIX.len(), data.len());
            return Err(format!("Unexpected prefix: {}", &data[0..ml]).into())
        }
        data = &data[PREFIX.len()..];

        let fields: Vec<&str> = data.split(',').collect();
        if fields.len() != 12 {
            return Err(format!("Expected 12 CSV elements, got {}", fields.len()).into());
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
            return Err("fixed0_1 is not 0".into());
        }
        if fixed0_2 != 0 {
            return Err("fixed0_2 is not 0".into());
        }

        Ok(Self {
            layout,
            icons: icons.try_into().unwrap(),
            timestamp,
            game_version: game_version.into(),
            short_desc: short_desc.into(),
            b64data_hash: b64data_hash.into(),
        })
    }

    pub fn into_bp_string(&self) -> String {
        let icons = self.icons.map(|x| x.to_string()).join(",");
        format!("BLUEPRINT:0,{},{},0,{},{},{},{}",
                self.layout, icons, self.timestamp, self.game_version, self.short_desc, self.b64data_hash)
    }
}
