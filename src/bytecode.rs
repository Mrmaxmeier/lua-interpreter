use header::Header;
use function_block::FunctionBlock;
use parser::*;
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
    use test::Bencher;
    use upvalues::Upvalue;
    use std::io::Cursor;
    use parser::Parsable;
    use instructions;
    use instructions::Instruction;
    use types::{Type, Number};
    use regex::Regex;

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../fixtures/assignment");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_if_conditions() {
        let data = include_bytes!("../fixtures/if_conditions");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_a_bunch_of_constants_correctly() {
        let data = include_bytes!("../fixtures/a_bunch_of_constants");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;


        assert_eq!(result.source_name.unwrap(), "@a_bunch_of_constants.lua".to_owned());
        assert_eq!(result.constants, vec![
            Type::Number(Number::Integer(42)),
            Type::Number(Number::Float(-0.08333333333)),
            Type::String("TSHRSTR".to_owned())
        ]);
    }

    #[ignore]
    #[test]
    fn parses_gcd() {
        let data = include_bytes!("../fixtures/gcd");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_assertions() {
        let data = include_bytes!("../fixtures/assertions");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_call_correctly() {
        let data = include_bytes!("../fixtures/call");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;
        println!("result: {:#?}\n", result);
        assert_eq!(result.source_name.unwrap(), "@call.lua".to_owned());
        assert_eq!(result.constants, vec![
            Type::String("print".into()),
            Type::String("value".into())
        ]);
    }

    #[test]
    fn parses_block_correctly() {
        let data = include_bytes!("../fixtures/block");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;

        println!("result: {:#?}\n", result);

        assert_eq!(result.source_name.unwrap(), "@block.lua".to_owned());
        assert_eq!(result.lines, (0, 0));
        assert_eq!(result.amount_parameters, 0);
        assert_eq!(result.stack_size, 2);
        assert_eq!(result.instructions, vec![
            Instruction::RETURN(instructions::Return{a: 0, b: 1})
        ]);
        assert_eq!(result.constants, vec![]);
        assert_eq!(result.protos, vec![]);
        assert_eq!(result.upvalues, vec![
            Upvalue {
                name: Some("_ENV".to_owned()),
                instack: true,
                index: 0
            }
        ]);
        assert!(result.debug.is_some());
    }

    #[bench]
    fn parse_a_bunch_of_constants(b: &mut Bencher) {
        let data = include_bytes!("../fixtures/a_bunch_of_constants").to_vec();
        b.iter(||
            Bytecode::parse(&mut Cursor::new(data.clone()))
        )
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
    fn pretty_prints_hello_world() {
        let data = include_bytes!("../fixtures/hello_world");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut stream = Cursor::new(Vec::new());
        result.pretty_print(&mut stream).unwrap();
        let pprint_result: String = String::from_utf8(stream.into_inner()).unwrap();
        println!("\n\n{}\n", pprint_result);
        let expected_lines = sanitized(r#"
main <@hello_world.lua> Lua (5, 3)

[4 instructions]
\t  1\t GetTabUp { reg: 0, upvalue: 0, constant: Constant(0) }   \t ; 0 = _ENV, 0 = "print"
\t  2\t LoadK { local: 1, constant: 1 }                          \t ; 1 = "Hello, World!"
\t  3\t Call { function: 0, params: Known(1), returns: Known(0) }\t ; 
\t  4\t Return { a: 0, b: 1 }                                    \t ; no return values

[2 constants]
\t  1\t  "print"
\t  2\t  "Hello, World!"

[0 locals]

[1 upvalue]
\t  1\t  Upvalue { name: Some("_ENV"), instack: true, index: 0 }

"#);
        let result_lines = pprint_result.lines()
            .map(|s| s.to_owned())
            .collect();
        assert_multiline_eq(result_lines, expected_lines);
    }


    #[ignore]
    #[test]
    fn pretty_prints_a_bunch_of_constants() {
        let data = include_bytes!("../fixtures/a_bunch_of_constants");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut stream = Cursor::new(Vec::new());
        result.pretty_print(&mut stream).unwrap();
        let pprint_result: String = String::from_utf8(stream.into_inner()).unwrap();
        println!("\n\n{}\n", pprint_result);
        let expected_lines = sanitized(r#"
main <@a_bunch_of_constants.lua> Lua (5, 3)

[6 instructions]
\t  1\t LoadNil { start: 0, range: 0 }               \t ; 0 = a
\t  2\t LoadBool { reg: 0, value: true, jump: false }\t ; 0 = a
\t  3\t LoadK { local: 0, constant: 0 }              \t ; 0 = a, 0 = 42
\t  4\t LoadK { local: 0, constant: 1 }              \t ; 0 = a, 1 = -0.08333333333
\t  5\t LoadK { local: 0, constant: 2 }              \t ; 0 = a, 2 = "TSHRSTR"
\t  6\t Return { a: 0, b: 1 }                        \t ; no return values

[3 constants]
\t  1\t  42
\t  2\t  -0.08333333333
\t  3\t  "TSHRSTR"

[1 local]
\t  1\t  Local { varname: "a", startpc: 1, endpc: 6 }

"#);
        let result_lines = pprint_result.lines()
            .map(|s| s.to_owned())
            .collect();
        assert_multiline_eq(result_lines, expected_lines);
    }

    #[ignore]
    #[test]
    fn pretty_if_conditions() {
        let data = include_bytes!("../fixtures/if_conditions");
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
