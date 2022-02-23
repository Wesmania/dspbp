use struct_deser::{FromBytes, IntoBytes};

use crate::error::Error;

pub struct Deser<'a>(&'a [u8]);

impl<'a> Deser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self(input)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn read_type<T: FromBytes>(&mut self) -> Result<T, Error> {
        if self.0.len() < T::BYTE_LEN {
            Err(Error::E("Input too short".into()))
        } else {
            let res = T::from_bytes(&self.0[..T::BYTE_LEN]);
            self.0 = &self.0[T::BYTE_LEN..];
            Ok(res)
        }
    }

    pub fn skip(&mut self, n: usize) -> Result<&[u8], Error> {
        if self.0.len() < n {
            Err(Error::E("Input too short".into()))
        } else {
            let ret = &self.0[..n];
            self.0 = &self.0[n..];
            Ok(ret)
        }
    }
}

pub struct Ser(Vec<u8>);
impl Ser {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn data(self) -> Vec<u8> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn write_type<T: IntoBytes>(&mut self, t: &T) {
        let mut din = vec![0; T::BYTE_LEN];
        t.into_bytes(&mut din);
        self.0.append(&mut din);
    }

    pub fn pad(&mut self, count: usize) {
        let l = self.0.len();
        self.0.resize(l + count, 0);
    }

    pub fn append(&mut self, data: &[u8]) {
        self.0.extend_from_slice(data);
    }
}
