use bytecode::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    version: (u8, u8),
    format_version: u8,
    size_of_int: u8,
    size_of_size_t: u8,
    size_of_instruction: u8,
    size_of_integer: u8,
    size_of_number: u8,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: (5, 3),
            format_version: 0,
            size_of_int: 4,
            size_of_size_t: 8,
            size_of_instruction: 4,
            size_of_integer: 8,
            size_of_number: 8,
        }
    }
}

named!(pub parse_header<Header>, chain!(
    tag!(LUA_SIGNATURE) ~
    v: tag!(&[0x53])    ~ // VERSION
    f: tag!(&[0x00])    ~ // FORMAT VERSION
    tag!(LUAC_DATA)     ~
    s_i: take!(1)       ~ // sizeof(int)
    s_st: take!(1)      ~ // sizeof(size_t)
    s_op: take!(1)      ~ // sizeof(Instruction)
    s_li: take!(1)      ~ // sizeof(lua_Integer)
    s_ln: take!(1)      ~ // sizeof(lua_Number)
    tag!(LUAC_INT)      ~
    tag!(&[0, 0, 0, 0]) , // TODO: make sure to consume correct amount of bytes
    || { Header {
        version: (v[0] >> 4, v[0] & 0xF),
        format_version: f[0],
        size_of_int: s_i[0],
        size_of_size_t: s_st[0],
        size_of_instruction: s_op[0],
        size_of_integer: s_li[0],
        size_of_number: s_ln[0],
    } }
));


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::{IResult, Needed};

    #[test]
    fn parses_header() {
        let data = &include_bytes!("../../fixtures/assignment")[..32];
        let expected = Header::default();

        let remaining = &data[29..];

        let result = parse_header(data);
        println!("{:?}\n", result);

        assert_eq!(result, IResult::Done(remaining, expected));
    }


    #[test]
    fn header_incomplete() {
        let data = &include_bytes!("../../fixtures/assignment")[..24];
        let result = parse_header(data);
        assert!(result.is_incomplete());
    }
}
