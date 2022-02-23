pub fn to32le<T: AsRef<[u8]>>(v: T) -> Vec<u32> {
    assert!(v.as_ref().len() % 4 == 0);
    v.as_ref()
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}

pub fn from32le<T: AsRef<[u32]>>(v: T) -> Vec<u8> {
    v.as_ref()
        .into_iter()
        .flat_map(|b| b.to_le_bytes().into_iter())
        .collect()
}
