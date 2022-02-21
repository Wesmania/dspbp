use std::fmt::Write as _;
use std::str::FromStr;
use std::io::{Read, Write};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use crate::bpdata::BlueprintData;
use crate::error::{Error, some_error};

use crate::md5::{MD5Hash, Algo, MD5};

#[derive(Serialize, Deserialize)]
pub struct Blueprint {
    layout: u32,
    icons: [u32; 5],
    timestamp: u64,
    game_version: String,
    short_desc: String,
    data: BlueprintData,
}

impl Blueprint {
    fn int<T: FromStr>(data: &str, what: &str) -> Result<T, Error> {
        str::parse(data).map_err(|_| format!("Failed to parse {}", what).into())
    }

    fn unpack_data(b64data: &str) -> anyhow::Result<BlueprintData> {
        let b64data = &b64data[1..];     // Skip first quote

        let zipped_data = base64::decode(b64data).map_err(|_| some_error("Failed to base64 decode blueprint"))?;
        let mut d = GzDecoder::new(zipped_data.as_slice());
        let mut data = vec![];
        d.read_to_end(&mut data)?;

        BlueprintData::new_from_buf(&data)
    }

    fn hash_str_to_hash(d: &str) -> anyhow::Result<MD5Hash> {
        if d.len() != 32 {
            return Err(some_error(format!("Unexpected hash length, expected 32, got {}", d.len())));
        }
        Ok((0..16)
            .map(|x| (2 * x .. 2 * x + 2))
            .map(|x| &d[x])
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<_>, _>>()?.try_into().unwrap())
    }

    fn hash(data: &str) -> MD5Hash {
        MD5::new(Algo::MD5F).process(data.as_bytes())
    }

    fn pack_data(&self) -> anyhow::Result<String> {
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        let data = self.data.write()?;
        e.write_all(&data).unwrap();
        let gzipped_data = e.finish().unwrap();
        Ok(base64::encode(gzipped_data.as_slice()))
    }

    pub fn new(data: &str) -> anyhow::Result<Self> {
        let data_and_hash: Vec<&str> = data.rsplitn(2, "\"").collect();
        if data_and_hash.len() != 2 {
            return Err(some_error("Did not find hash delimiter"));
        }
        let [hash, mut data]: [&str; 2] = data_and_hash.try_into().unwrap();

        // NOTICE: we hash the blueprint without the trailing quote!
        let hash = Self::hash_str_to_hash(hash)?;
        let our_hash = Self::hash(data);
        if hash != our_hash {
            return Err(some_error(format!("Blueprint hash does not match calculated hash: {:x?} != {:x?}", hash, our_hash)));
        }

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
        let [fixed0_2, timestamp, game_version, short_desc, b64data]: [&str; 5] = fields[7..12].try_into().unwrap();

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

        let data = Self::unpack_data(b64data)?;

        Ok(Self {
            layout,
            icons: icons.try_into().unwrap(),
            timestamp,
            game_version: game_version.into(),
            short_desc: short_desc.into(),
            data,
        })
    }

    pub fn into_bp_string(&self) -> anyhow::Result<String> {
        let icons = self.icons.map(|x| x.to_string()).join(",");
        let mut out = format!("BLUEPRINT:0,{},{},0,{},{},{},\"{}",
                self.layout, icons, self.timestamp, self.game_version,
                self.short_desc, self.pack_data()?);
        let hash = Self::hash(&out);
        write!(&mut out, "\"").unwrap();
        for b in hash {
            write!(&mut out, "{:02X}", b).unwrap();
        }
        Ok(out)
    }

    pub fn new_from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn dump_json(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}
