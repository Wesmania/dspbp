use std::{fs::File, io::Read, path::PathBuf};

pub fn get_file(f: &str) -> Vec<u8> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples");
    p.push(f);
    let mut res = Vec::new();
    File::open(p).unwrap().read_to_end(&mut res).unwrap();
    res
}
