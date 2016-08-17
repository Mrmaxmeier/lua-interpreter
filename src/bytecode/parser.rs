pub use std::io;
pub use std::io::Read;
use byteorder;
pub use byteorder::ReadBytesExt;

pub const LUA_SIGNATURE: &'static [u8] = &[0x1B, b'L', b'u', b'a'];
pub const LUAC_DATA: &'static [u8] = &[0x19, 0x93, b'\r', b'\n', 0x1a, b'\n'];
pub const LUAC_INT: Integer = 0x5678;
pub const LUAC_NUM: Float = 370.5;


pub trait Parsable: Sized {
    fn parse<R: Read + Sized>(&mut R) -> Self;
}

impl Parsable for String {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        r.parse_lua_string().unwrap()
    }
}

impl Parsable for u8 {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let mut buf = [0u8];
        r.read_exact(&mut buf).unwrap();
        buf[0]
    }
}

impl Parsable for i32 {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        r.read_i32::<byteorder::LittleEndian>().unwrap()
    }
}

impl Parsable for i64 {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        r.read_i64::<byteorder::LittleEndian>().unwrap()
    }
}

impl Parsable for f64 {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        r.read_f64::<byteorder::LittleEndian>().unwrap()
    }
}

pub type Integer = i32;
pub type Float = f64;

pub trait ReadExt: Read + Sized {
    fn assert_byte(&mut self, byte: u8) {
        let read = u8::parse(self);
        assert_eq!(read, byte);
    }

    fn assert_bytes(&mut self, bytes: &[u8]) {
        let mut read = vec![0u8; bytes.len()];
        self.read_exact(&mut read).unwrap();
        assert_eq!(read, bytes);
    }

    fn read_byte(&mut self) -> u8 {
        let mut buf = [0u8];
        self.read_exact(&mut buf).unwrap();
        buf[0]
    }

    fn read_bytes(&mut self, amount: usize) -> Vec<u8> {
        let mut buf = vec![0u8; amount];
        self.read_exact(&mut buf).unwrap();
        buf
    }

    fn parse_lua_string(&mut self) -> Option<String> {
        let len = match self.read_byte() {
            0x00 => return None,
            0xFF => {
                let n = Integer::parse(self) as usize;
                println!("parsing long string ({}) due to 0xFF", n);
                n
            },
            byte => byte as usize,
        };
        println!("string size: {}", len);
        let data = self.read_bytes(len - 1);
        Some(String::from_utf8_lossy(&data).into_owned())
    }
}

impl<R: Read + Sized> ReadExt for R {}