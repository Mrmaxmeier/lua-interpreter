use std::cmp::max;
use types::Type;
use nom;
use nom::le_i32;

pub const LUA_SIGNATURE: &'static [u8] = &[0x1B, b'L', b'u', b'a'];
pub const LUAC_DATA: &'static [u8] = &[0x19, 0x93, b'\r', b'\n', 0x1a, b'\n'];
pub const LUAC_INT: &'static [u8] = &[
    0x78, 0x56, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
pub const LUAC_NUM: &'static [u8] = &[
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x28, 0x77, 0x40,
]; 



bitflags! {
    pub flags VarArgs: u8 {
        const VARARG_HASARG   = 0b0001,
        const VARARG_ISVARARG = 0b0010,
        const VARARG_NEEDSARG = 0b0100,
        const VARARG_DEFAULT  = VARARG_HASARG.bits
                              | VARARG_ISVARARG.bits
                              | VARARG_NEEDSARG.bits,
    }
}


named!(pub parse_string< Option<String> >, chain!(
    len: alt!(
        chain!(tag!(&[0xFF]) ~ l: call!(le_i32), {|| l as u32}) |
        take!(1) => {|v: &[u8]| v[0] as u32}
    ) ~
    data: take!(max(1, len) - 1),
    || {
        if len > 0 {
            Some(String::from_utf8_lossy(data).into_owned())
        } else {
            None
        }
    }
));

named!(pub parse_int<i32>, call!(le_i32));