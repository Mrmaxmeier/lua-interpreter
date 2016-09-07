use parser::*;
use code::Code;
use constants::Constants;
use upvalues::Upvalues;
use debug::Debug;
use instruction::InstructionContext;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    pub source_name: Option<String>,
    pub lines: (usize, usize),
    pub amount_parameters: u8,
    pub stack_size: u8,
    pub instructions: Code,
    pub constants: Constants,
    pub protos: Vec<FunctionBlock>,
    pub upvalues: Upvalues,
    pub debug: Debug
}

impl FunctionBlock {
    fn propagate_source(&mut self, name: Option<String>) {
        for proto in &mut self.protos {
            proto.propagate_source(name.clone());
        }
        if self.source_name.is_none() {
            self.source_name = name;
        }
    }

    fn pretty_section<W: io::Write + Sized>(&self, w: &mut W, namebase: &str, count: usize) -> io::Result<()> {
        let s = if count != 1 { "s" } else { "" };
        writeln!(w, "\n[{} {}{}]", count, namebase, s)
    }

    pub fn pretty_print<W: io::Write + Sized>(&self, w: &mut W) -> io::Result<()> {
        self.pretty_section(w, "instruction", self.instructions.len())?;
        if let Some(ref debug) = self.debug {
            let debug_fmts: Vec<String> = self.instructions.iter()
                .map(|instr| format!("{:?}", instr.as_ops()))
                .collect();
            let max_length = debug_fmts.iter()
                .map(|s| s.len())
                .max()
                .unwrap();
            for (i, instr) in self.instructions.iter().enumerate() {
                let debug_fmt = format!("{:?}", instr.as_ops());
                let padding = ::std::iter::repeat(' ')
                    .take(max_length - debug_fmt.len())
                    .collect::<String>();
                let context = InstructionContext {
                    index: i,
                    debug: debug,
                    func: self,
                };
                let debug_info = instr.as_ops().debug_info(context).join(", ");
                writeln!(w, "\t{}\t{}{}\t; {}", i + 1, debug_fmt, padding, debug_info)?;
            }
        } else {
            for (i, instr) in self.instructions.iter().enumerate() {
                writeln!(w, "\t{}\t{:?}", i + 1, instr.as_ops())?;
            }
        }
        self.pretty_section(w, "constant", self.constants.len())?;
        for (i, constant) in self.constants.iter().enumerate() {
            writeln!(w, "\t{}\t{}", i + 1, constant.repr())?;
        };
        if let Some(ref debug) = self.debug {
            self.pretty_section(w, "local", debug.locals.len())?;
            for (i, local) in debug.locals.iter().enumerate() {
                writeln!(w, "\t{}\t{:?}", i + 1, local)?;
            };
        };
        self.pretty_section(w, "upvalue", self.upvalues.len())?;
        for (i, upvalue) in self.upvalues.iter().enumerate() {
            writeln!(w, "\t{}\t{:?}", i + 1, upvalue)?;
        };
        for subblock in &self.protos {
            subblock.pretty_print(w)?;
        }
        Ok(())
    }
}

impl Parsable for FunctionBlock {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let source_name = r.parse_lua_string();
        // println!("source_name: {:?}", source_name);
        let lines = (u32::parse(r) as usize, u32::parse(r) as usize);
        let params = u8::parse(r);
        let is_vararg = u8::parse(r);
        match is_vararg {
            0 | 2 => {},
            _ => unimplemented!()
        }
        let stack_size = u8::parse(r);
        // println!("stack_size: {:?}", stack_size);

        let code = Code::parse(r);
        // println!("code {:#?}", code);
        let constants = Constants::parse(r);
        // println!("constants {:#?}", constants);
        let mut upvalues = Upvalues::parse(r);
        // println!("upvalues {:#?}", upvalues);
        let len_protos = u32::parse(r);
        // println!("parsing {} protos (subblocks)", len_protos);
        let mut protos = (0..len_protos)
            .map(|_| FunctionBlock::parse(r))
            .collect::<Vec<_>>();
        for proto in &mut protos {
            proto.propagate_source(source_name.clone());
        }
        let debug = Debug::parse(r);
        if let Some(ref debug_data) = debug {
            // println!("debug: {:#?}", debug_data);
            debug_data.update_upvalues(&mut upvalues);
        }

        FunctionBlock {
            source_name: source_name,
            lines: lines,
            amount_parameters: params,
            stack_size: stack_size,
            instructions: code,
            constants: constants,
            upvalues: upvalues,
            protos: protos,
            debug: debug
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use types::Type;
    use header::Header;
    use instructions;
    use instruction::Instruction;
    use upvalues::Upvalue;
    use parser::{Parsable, ReadExt};

    #[test]
    fn parses_assignment() {
        let all = include_bytes!("../fixtures/assignment");
        let mut reader = Cursor::new(all.to_vec());
        Header::parse(&mut reader);
        reader.read_byte(); // skip count of upvalues
        assert_eq!(34, reader.position());

        let result = FunctionBlock::parse(&mut reader);
        println!("result: {:#?}\n", result);

        assert_eq!(result.source_name, Some("@assignment.lua".to_owned()));
        assert_eq!(result.lines, (0, 0));
        assert_eq!(result.stack_size, 2);
        assert_eq!(result.instructions, vec![
            Instruction::LOADK(instructions::LoadK {local: 0, constant: 0}),
            Instruction::RETURN(instructions::Return {a: 0, b: 1}),
        ]);
        assert_eq!(result.constants, vec![
            Type::String("zweiundvierzig".into())
        ]);
        assert_eq!(result.upvalues, vec![
            Upvalue {
                name: Some("_ENV".into()),
                instack: true,
                index: 0
            }
        ]);
    }
}
