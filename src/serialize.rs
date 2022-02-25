use std::io::{Write, Read};

use struct_deser::{FromBytes, IntoBytes};

pub trait ReadType {
    fn read_type<T: FromBytes>(&mut self) -> anyhow::Result<T>;
    fn skip(&mut self, count: usize) -> anyhow::Result<()>;
}

pub trait WriteType {
    fn write_type<T: IntoBytes>(&mut self, t: &T) -> anyhow::Result<()>;
    fn pad(&mut self, count: usize) -> anyhow::Result<()>;
}

impl<R: Read> ReadType for R {
    fn read_type<T: FromBytes>(&mut self) -> anyhow::Result<T> {
        let mut data = vec![0u8; T::BYTE_LEN];
        self.read_exact(&mut data)?;
        Ok(T::from_bytes(&data))
    }

    fn skip(&mut self, mut count: usize) -> anyhow::Result<()> {
        let buf = &mut [0u8; 64];
        while count > 0 {
            let d = std::cmp::min(count, 64);
            self.read_exact(&mut buf[..d])?;
            count -= d;
        }
        Ok(())
    }
}

impl<W: Write> WriteType for W {
    fn write_type<T: IntoBytes>(&mut self, t: &T) -> anyhow::Result<()> {
        let mut din = vec![0; T::BYTE_LEN];
        t.into_bytes(&mut din);
        self.write_all(&mut din)?;
        Ok(())
    }

    fn pad(&mut self, mut count: usize) -> anyhow::Result<()> {
        let buf = &mut [0u8; 64];
        while count > 0 {
            let d = std::cmp::min(count, 64);
            self.write_all(&mut buf[..d])?;
            count -= d;
        }
        Ok(())
    }
}
