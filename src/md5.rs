use lazy_static::lazy_static;
use std::collections::HashMap;

const K: &[u32; 64] = &[
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

lazy_static! {
    static ref K_MD5F: HashMap<usize, u32> = HashMap::from([
            (1, 0xe8d7b756),
            (6, 0xa8304623),
            (12, 0x6b9f1122),
            (15, 0x39b40821),
            (19, 0xc9b6c7aa),
            (21, 0x02443453),
            (24, 0x21f1cde6),
            (27, 0x475a14ed),
    ]);
    static ref K_MD5FC: HashMap<usize, u32> = HashMap::from([
            (1, 0xe8d7b756),
            (3, 0xc1bdceef),
            (6, 0xa8304623),
            (12, 0x6b9f1122),
            (15, 0x39b40821),
            (19, 0xc9b6c7aa),
            (21, 0x02443453),
            (24, 0x23f1cde6),
            (27, 0x475a14ed),
            (34, 0x6d9d6121),
    ]);
}

const S: &[usize; 64] = &[
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
];

const INIT_MD5: [u32; 4] = [
    u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
    u32::from_le_bytes([0x89, 0xab, 0xcd, 0xef]),
    u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
    u32::from_le_bytes([0x76, 0x54, 0x32, 0x10]),
];

const INIT_MD5F: [u32; 4] = [
    u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
    u32::from_le_bytes([0x89, 0xab, 0xdc, 0xef]),
    u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
    u32::from_le_bytes([0x46, 0x57, 0x32, 0x10]),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Algo {
    MD5,
    MD5F,
    MD5FC,
}

pub struct MD5 {
    s: [u32; 4],
    algo: Algo,
}

pub type MD5Hash = [u8; 16];

impl MD5 {
    pub fn new(algo: Algo) -> Self {
        let s = match algo {
            Algo::MD5 => INIT_MD5,
            _ => INIT_MD5F,
        };

        Self {
            s,
            algo,
        }
    }

    fn k(&self, i: usize) -> u32 {
        let u = if self.algo == Algo::MD5F && K_MD5F.contains_key(&i) {
            *K_MD5F.get(&i).unwrap()
        } else if self.algo == Algo::MD5FC && K_MD5FC.contains_key(&i) {
            *K_MD5FC.get(&i).unwrap()
        } else {
            K[i]
        };
        u32::from_le(u)
    }

    fn update_block(&mut self, buf: &[u8]) {
        assert!(buf.len() == 64);
        let words: Vec<u32> = buf.chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
        let mut a = self.s[0];
        let mut b = self.s[1];
        let mut c = self.s[2];
        let mut d = self.s[3];
        
        for i in 0..64 {
            let mut f: u32;
            let g: usize;
            if i < 16 {
                f = (b & c) | (!b & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            }

            f = f.wrapping_add(a).wrapping_add(self.k(i as usize)).wrapping_add(words[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i as usize] as u32));
        }
        self.s[0] = self.s[0].wrapping_add(a);
        self.s[1] = self.s[1].wrapping_add(b);
        self.s[2] = self.s[2].wrapping_add(c);
        self.s[3] = self.s[3].wrapping_add(d);
    }

    pub fn process(&mut self, data: &[u8]) -> MD5Hash {
        let chunks = data.chunks_exact(64);
        let mut last: Vec<u8> = chunks.remainder().into();
        for chunk in chunks {
            self.update_block(chunk);
        }
        last.push(0x80);
        while last.len() % 64 != 56 {
            last.push(0x0);
        }
        last.extend_from_slice(&(data.len() as u64 * 8).to_le_bytes());
        for chunk in last.chunks_exact(64) {
            self.update_block(chunk);
        }

        let mut out: MD5Hash = [0; 16];
        (&mut out[0..4]).copy_from_slice(&self.s[0].to_le_bytes());
        (&mut out[4..8]).copy_from_slice(&self.s[1].to_le_bytes());
        (&mut out[8..12]).copy_from_slice(&self.s[2].to_le_bytes());
        (&mut out[12..16]).copy_from_slice(&self.s[3].to_le_bytes());
        out
    }
}

#[cfg(test)]
mod test {
    use crate::md5::MD5Hash;

    use super::{MD5, Algo};

    #[test]
    fn test_md5_empty() {
        let hash = MD5::new(Algo::MD5).process(&[]);
        let expected: MD5Hash = [0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5_reference() {
        let text = "12345678901234567890123456789012345678901234567890123456789012345678901234567890";
        let hash = MD5::new(Algo::MD5).process(text.as_bytes());
        let expected: MD5Hash = [0x57, 0xed, 0xf4, 0xa2, 0x2b, 0xe3, 0xc9, 0x55, 0xac, 0x49, 0xda, 0x2e, 0x21, 0x07, 0xb6, 0x7a];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5f_empty() {
        let hash = MD5::new(Algo::MD5F).process(&[]);
        let expected: MD5Hash = [0x84, 0xd1, 0xce, 0x3b, 0xd6, 0x8f, 0x49, 0xab, 0x26, 0xeb, 0x0f, 0x96, 0x41, 0x66, 0x17, 0xcf];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5f_a() {
        let text = "a";
        let hash = MD5::new(Algo::MD5F).process(text.as_bytes());
        let expected: MD5Hash = [0xf1, 0x0b, 0xdd, 0xae, 0xcb, 0x62, 0xe5, 0xa9, 0x24, 0x33, 0x75, 0x78, 0x67, 0xee, 0x06, 0xdb];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5f_abcd() {
        let text = "abcd";
        let hash = MD5::new(Algo::MD5F).process(text.as_bytes());
        let expected: MD5Hash = [0xfa, 0x27, 0xc7, 0x8b, 0x6e, 0xc3, 0x15, 0x59, 0xf0, 0xe7, 0x60, 0xce, 0x3f, 0x2b, 0x03, 0xf6];
        assert!(hash == expected);
    }
}
