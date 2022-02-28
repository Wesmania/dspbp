use std::io::{Write, Read};

pub trait ReadType {
    fn skip(&mut self, count: usize) -> anyhow::Result<()>;
}

pub trait WriteType {
    fn pad(&mut self, count: usize) -> anyhow::Result<()>;
}

impl<R: Read> ReadType for R {
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
