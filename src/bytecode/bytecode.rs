use bytecode::header::Header;
use bytecode::function_block::FunctionBlock;
use bytecode::parser::*;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub header: Header,
    pub upvalues: u8,
    pub func: FunctionBlock,
}

impl Bytecode {
    pub fn pretty_print<W: Write + Sized>(&self, w: &mut W) -> io::Result<()> {
        if let Some(ref name) = self.func.source_name {
            write!(w, "main <{}> ", name)?;
        }
        writeln!(w, "Lua {:?}", self.header.version)?;
        self.func.pretty_print(w)
    }
}

impl Parsable for Bytecode {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        Bytecode {
            header: Header::parse(r),
            upvalues: u8::parse(r),
            func: FunctionBlock::parse(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::upvalues::Upvalue;
    use std::io::Cursor;
    use bytecode::parser::Parsable;
    use bytecode::instructions;
    use bytecode::instructions::Instruction;
    use types::{Type, Number};
    use regex::Regex;

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_if_conditions() {
        let data = include_bytes!("../../fixtures/if_conditions");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_a_bunch_of_constants_correctly() {
        let data = include_bytes!("../../fixtures/a_bunch_of_constants");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;


        assert_eq!(result.source_name.unwrap(), "@a_bunch_of_constants.lua".to_owned());
        assert_eq!(result.constants, vec![
            box Type::Number(Number::Integer(42)),
            box Type::Number(Number::Float(-0.08333333333)),
            box Type::String("TSHRSTR".to_owned()),
            box Type::String(
                "TLNGSTR\n\
______________________________________50\n\
_____________________________________100\n\
_____________________________________150\n\
_____________________________________200\n\
_____________________________________250\n\
_____________________________________300".to_owned()
            )
        ]);
    }

    #[ignore]
    #[test]
    fn parses_gcd() {
        let data = include_bytes!("../../fixtures/gcd");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_call_correctly() {
        let data = include_bytes!("../../fixtures/call");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;
        println!("result: {:#?}\n", result);
        assert_eq!(result.source_name.unwrap(), "@call.lua".to_owned());
        assert_eq!(result.constants, vec![
            box Type::String("print".into()),
            box Type::String("value".into())
        ]);
    }

    #[test]
    fn parses_block_correctly() {
        let data = include_bytes!("../../fixtures/block");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;

        println!("result: {:#?}\n", result);

        assert_eq!(result.source_name.unwrap(), "@block.lua".to_owned());
        assert_eq!(result.lines, (0, 0));
        assert_eq!(result.amount_parameters, 0);
        assert_eq!(result.stack_size, 2);
        assert_eq!(result.instructions, vec![
            box Instruction::RETURN(instructions::Return{a: 0, b: 0})
        ]);
        assert_eq!(result.constants, vec![]);
        assert_eq!(result.protos, vec![]);
        assert_eq!(result.upvalues, vec![
            Upvalue {
                name: Some("_ENV".to_owned()),
                instack: 1,
                idx: 0
            }
        ]);
        assert!(result.debug.is_some());
    }

    fn sanitized(s: &str) -> Vec<String> {
        let s = s.to_owned();
        let lines = s.lines().count();

        let re = Regex::new(r"\\t +").unwrap();
        s.lines()
            .skip(1)
            .take(lines - 2)
            .map(|s| re.replace_all(s, "\t"))
            .collect()
    }

    fn assert_multiline_eq(a: Vec<String>, b: Vec<String>) {
        for (line_r, line_e) in a.iter().zip(&b) {
            if line_r != line_e {
                println!("result:   {}", line_r);
                println!("expected: {}", line_e);
                println!("result:   {:?}", line_r);
                println!("expected: {:?}", line_e);
            }
            assert_eq!(line_r, line_e);
        }
        assert_eq!(a.len(), b.len(), "line count mismatch");
    }

    #[test]
    fn pretty_prints_a_bunch_of_constants() {
        let data = include_bytes!("../../fixtures/a_bunch_of_constants");
        let mut result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        result.func.constants[3] = box Type::String("TLNGSTR".into());
        let mut stream = Cursor::new(Vec::new());
        result.pretty_print(&mut stream).unwrap();
        let pprint_result: String = String::from_utf8(stream.into_inner()).unwrap();
        println!("\n\n{}\n", pprint_result);
        let expected_lines = sanitized(r#"
main <@a_bunch_of_constants.lua> Lua (5, 3)

[7 instructions]
\t  1\t LoadNil { start: 0, range: 0 }               \t ; 0 = a
\t  2\t LoadBool { reg: 0, value: true, jump: false }\t ; 0 = a
\t  3\t LoadK { local: 0, constant: 0 }              \t ; 0 = a, 0 = 42
\t  4\t LoadK { local: 0, constant: 1 }              \t ; 0 = a, 1 = -0.08333333333
\t  5\t LoadK { local: 0, constant: 2 }              \t ; 0 = a, 2 = "TSHRSTR"
\t  6\t LoadK { local: 0, constant: 3 }              \t ; 0 = a, 3 = "TLNGSTR"
\t  7\t Return { a: 0, b: 0 }                        \t ; return to top

[4 constants]
\t  1\t  42
\t  2\t  -0.08333333333
\t  3\t  "TSHRSTR"
\t  4\t  "TLNGSTR"

[1 locals]
\t  1\t  Local { varname: "a", startpc: 1, endpc: 7 }

"#);
        let result_lines = pprint_result.lines()
            .map(|s| s.to_owned())
            .collect();
        assert_multiline_eq(result_lines, expected_lines);
    }

    #[test]
    fn pretty_if_conditions() {
        let data = include_bytes!("../../fixtures/if_conditions");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut stream = Cursor::new(Vec::new());
        result.pretty_print(&mut stream).unwrap();
        let pprint_result: String = String::from_utf8(stream.into_inner()).unwrap();
        println!("\n\n{}\n", pprint_result);
        let expected_lines = sanitized(r#"
main <@if_conditions.lua> Lua (5, 3)

[18 instructions]
\t  1\t GetTabUp { reg: 0, upvalue: 0, constant: Constant(0) }\t ; 0 = _ENV, 0 = "print"
\t  2\t LoadK { local: 1, constant: 1 }                       \t ; 1 = "true is truthy"

"#);
        let result_lines = pprint_result.lines()
            .map(|s| s.to_owned())
            .collect();
        assert_multiline_eq(result_lines, expected_lines);
    }
}
