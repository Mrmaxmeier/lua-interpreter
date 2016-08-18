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
        writeln!(w, "\n[{} instructions]", self.func.instructions.len())?;
        for (i, instr) in self.func.instructions.iter().enumerate() {
            writeln!(w, "\t{}\t{:?}", i + 1, instr)?;
        }
        writeln!(w, "\n[{} constants]", self.func.constants.len())?;
        for (i, constant) in self.func.constants.iter().enumerate() {
            writeln!(w, "\t{}\t{}", i + 1, constant)?;
        };
        if let Some(ref debug) = self.func.debug {
            writeln!(w, "\n[{} locals]", debug.locals.len())?;
            for (i, local) in debug.locals.iter().enumerate() {
                writeln!(w, "\t{}\t{:?}", i + 1, local)?;
            };
        }
        Ok(())
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
\t  1\t LoadNil { a: 0, b: 0 }                        \t   ; a = "a"
\t  2\t LoadBool { reg: 0, value: false, jump: true })\t   ; local = "a"
\t  3\t LoadK { local: 0, constant: 0 }               \t   ; local = "a" constant = 42
\t  4\t LoadK { local: 0, constant: 1 }               \t   ; local = "a" constant = -0.08333333333
"#);
        for (line_r, line_e) in pprint_result.lines().zip(&expected_lines) {
            if line_r != line_e {
                println!("result:   {}", line_r);
                println!("expected: {}", line_e);
            }
            assert_eq!(line_r, line_e);
        }
        assert_eq!(pprint_result.lines().count(), expected_lines.len(), "line count mismatch");
    }
}
