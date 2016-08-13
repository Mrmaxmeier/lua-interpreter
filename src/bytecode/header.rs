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

impl Parsable for Header {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let h = Header::default();

        r.assert_bytes(LUA_SIGNATURE);

        let (v_major, v_minor) = h.version;
        r.assert_byte(v_major << 4 | v_minor);
        r.assert_byte(h.format_version);

        r.assert_bytes(LUAC_DATA);
        
        r.assert_byte(h.size_of_int);
        r.assert_byte(h.size_of_size_t);
        r.assert_byte(h.size_of_instruction);
        r.assert_byte(h.size_of_integer);
        r.assert_byte(h.size_of_number);

        r.assert_bytes(LUAC_INT);
        r.assert_bytes(LUAC_NUM);

        h
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_assignment_header() {
        let data = &include_bytes!("../../fixtures/assignment")[..40];
        let expected = Header::default();

        let remaining = &data[33..];

        let result = parse_header(data);
        println!("{:#?}\n", result);

        assert_eq!(result, IResult::Done(remaining, expected));
    }


    #[test]
    fn header_incomplete() {
        let data = &include_bytes!("../../fixtures/assignment")[..24];
        let result = parse_header(data);
        assert!(result.is_incomplete());
    }
}
