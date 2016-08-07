use types::Type;
use nom;

const LUA_SIGNATURE: &'static [u8] = &[0x1B, b'L', b'u', b'a'];
const LUAC_DATA: &'static [u8] = &[0x19, 0x93, b'\r', b'\n', 0x1a, b'\n'];
const LUAC_INT: &'static [u8] = &[0x78, 0x56];


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

named!(pub header<Header>, chain!(
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
    tag!(&[0x00])       ,
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

/*
  DumpLiteral(LUA_SIGNATURE, D);
  DumpByte(LUAC_VERSION, D);
  DumpByte(LUAC_FORMAT, D);
  DumpLiteral(LUAC_DATA, D);
  DumpByte(sizeof(int), D);
  DumpByte(sizeof(size_t), D);
  DumpByte(sizeof(Instruction), D);
  DumpByte(sizeof(lua_Integer), D);
  DumpByte(sizeof(lua_Number), D);
  DumpInteger(LUAC_INT, D);
  DumpNumber(LUAC_NUM, D);
*/

/*
Function block of a Lua 5 binary chunk
Holds all the relevant data for a function. There is one top-level function.
String source name
Integer line defined
Integer last line defined
1 byte number of upvalues
1 byte number of parameters
1 byte is_vararg flag (see explanation further below)
• 1=VARARG_HASARG
• 2=VARARG_ISVARARG
• 4=VARARG_NEEDSARG
1 byte maximum stack size (number of registers used)
List list of instructions (code)
List list of constants
List list of function prototypes
List source line positions (optional debug data)
List list of locals (optional debug data)
List list of upvalues (optional debug data)
*/


#[derive(Debug)]
struct FunctionBlock {
    source_name: String,
    lines: (usize, usize),
    amount_upvalues: u8,
    amount_parameters: u8,
    is_vararg: VarArgs,
    stack_size: u8,
    instructions: Vec<()>,
    constants: Vec<()>,
// DEBUG DATA
    source_line_positions: Vec<()>,
    locals: Vec<()>,
    upvalues: Vec<()>,
}

bitflags! {
    flags VarArgs: u8 {
        const VARARG_HASARG   = 0b0001,
        const VARARG_ISVARARG = 0b0010,
        const VARARG_NEEDSARG = 0b0100,
        const VARARG_DEFAULT  = VARARG_HASARG.bits
                              | VARARG_ISVARARG.bits
                              | VARARG_NEEDSARG.bits,
    }
}


/*
All strings are defined in the following format:
Size_t String data size
Bytes String data, includes a NUL (ASCII 0) at the end
The string data size takes into consideration a NUL character at the end,
so an empty string (“”) has 1 as the size_t value. A size_t of 0 means zero
string data bytes; the string does not exist. This is often used by the source
name field of a function.
*/
// named!(string)


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::IResult;

    #[test]
    fn parses_header() {
        let data = &[
            0x1B, 0x4C, 0x75, 0x61, 0x53, 0x00, 0x19, 0x93,
            0x0D, 0x0A, 0x1A, 0x0A, 0x04, 0x08, 0x04, 0x08,
            0x08, 0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let expected = Header {
            version: (5, 3),
            format_version: 0,
            size_of_int: 4,
            size_of_size_t: 8,
            size_of_instruction: 4,
            size_of_integer: 8,
            size_of_number: 8,
        };

        let remaining = &data[20..];

        let result = header(data);
        println!("{:?}\n", result);

        assert_eq!(result, IResult::Done(remaining, expected));
    }
}
