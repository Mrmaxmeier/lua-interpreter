// http://www.lua.org/source/5.3/lopcodes.h.html
use std::fmt;
use parser::*;
use function_block::FunctionBlock;
use debug::DebugData;
use byteorder;
pub use types::{Type, Representable};
pub use interpreter::Context;
pub use stack::{StackEntry, Stack};

use instructions::*;

macro_rules! on_bits {
    ($n:expr) => (((1 << $n) - 1))
}

macro_rules! get_bits {
    ($d:expr, $n:expr => $m:expr) => (
        ($d >> $n) & on_bits!(($m - $n))
    )
}

pub type Reg = usize;

pub trait LoadInstruction: Sized {
    fn load(u32) -> Self;
}

pub trait InstructionOps: fmt::Debug {
    fn exec(&self, _: &mut Context) {
        println!("exec not yet implemented for {:?}!", self);
        unimplemented!()
    } // TODO: remove impl
    fn debug_info(&self, InstructionContext) -> Vec<String> { vec![] }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MOVE(Move),
    LOADK(LoadK),
    LOADBOOL(LoadBool),
    LOADNIL(LoadNil),
    GETTABUP(GetTabUp),
    SETTABUP(SetTabUp),
    ADD(Add),
    SUB(Sub),
    MUL(Mul),
    MOD(Mod),
    POW(Pow),
    DIV(Div),
    IDIV(IDiv),
    BAND(BAnd),
    BOR(BOr),
    BXOR(BXor),
    SHL(Shl),
    SHR(Shr),
    UNM(Unm),
    BNOT(BNot),
    NOT(Not),
    LEN(Len),
    CONCAT(Concat),
    JMP(Jmp),
    TEST(Test),
    EQ(Equals),
    LT(LessThan),
    LE(LessThanOrEquals),
    CALL(Call),
    TAILCALL(Tailcall),
    RETURN(Return),
    FORLOOP(ForLoop),
    FORPREP(ForPrep),
    CLOSURE(Closure),
}

macro_rules! match_trait_as_impl {
    ( $this:expr, [$( $x:path ),*] => as $cast_to:ty ) => {
        match $this {
            $( &$x(ref v) => v as $cast_to, )*
            v => panic!("`as {}` not implemented for {:?}", stringify!($cast_to), v) 
        }
    };
}

impl Instruction {
    pub fn as_ops(&self) -> &InstructionOps {
        match_trait_as_impl!(self, [
            Instruction::MOVE,
            Instruction::LOADK,
            Instruction::LOADBOOL,
            Instruction::LOADNIL,
            Instruction::GETTABUP,
            Instruction::ADD,
            Instruction::SUB,
            Instruction::MUL,
            Instruction::MOD,
            Instruction::POW,
            Instruction::DIV,
            Instruction::IDIV,
            Instruction::BAND,
            Instruction::BOR,
            Instruction::BXOR,
            Instruction::SHL,
            Instruction::SHR,
            Instruction::UNM,
            Instruction::BNOT,
            Instruction::NOT,
            Instruction::LEN,
            Instruction::CONCAT,
            Instruction::JMP,
            Instruction::EQ,
            Instruction::LE,
            Instruction::LT,
            Instruction::TEST,
            Instruction::CALL,
            Instruction::RETURN,
            Instruction::FORLOOP,
            Instruction::FORPREP,
            Instruction::CLOSURE
        ] => as &InstructionOps)
    }
    pub fn exec(&self, i: &mut Context) {
        self.as_ops().exec(i);
    }
}


impl Parsable for Instruction {
    #[allow(unknown_lints)]
    #[allow(zero_prefixed_literal)]
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let data = r.read_u32::<byteorder::LittleEndian>().unwrap();
        let opcode = data & on_bits!(6);
        // println!("opcode: {:?}\tdata: 0b{:0>32b}", opcode, data);
        match opcode {
            00 => Instruction::MOVE(Move::load(data)),
            01 => Instruction::LOADK(LoadK::load(data)),
            // TODO: 2 LOADKX
            03 => Instruction::LOADBOOL(LoadBool::load(data)),
            04 => Instruction::LOADNIL(LoadNil::load(data)),
            // TODO: 5 GETUPVAL
            06 => Instruction::GETTABUP(GetTabUp::load(data)),
            // TODO: 7 GETTABLE
            08 => Instruction::SETTABUP(SetTabUp::load(data)),
            // TODO: 9 SETUPVAL
            // TODO: 10 SETTABLE
            // TODO: 11 NEWTABLE
            // TODO: 12 SELF
            13 => Instruction::ADD(Add::load(data)),
            14 => Instruction::SUB(Sub::load(data)),
            15 => Instruction::MUL(Mul::load(data)),
            16 => Instruction::MOD(Mod::load(data)),
            17 => Instruction::POW(Pow::load(data)),
            18 => Instruction::DIV(Div::load(data)),
            19 => Instruction::IDIV(IDiv::load(data)),
            20 => Instruction::BAND(BAnd::load(data)),
            21 => Instruction::BOR(BOr::load(data)),
            22 => Instruction::BXOR(BXor::load(data)),
            23 => Instruction::SHL(Shl::load(data)),
            24 => Instruction::SHR(Shr::load(data)),
            25 => Instruction::UNM(Unm::load(data)),
            26 => Instruction::BNOT(BNot::load(data)),
            27 => Instruction::NOT(Not::load(data)),
            28 => Instruction::LEN(Len::load(data)),
            29 => Instruction::CONCAT(Concat::load(data)),
            30 => Instruction::JMP(Jmp::load(data)),
            31 => Instruction::EQ(Equals::load(data)),
            32 => Instruction::LT(LessThan::load(data)),
            33 => Instruction::LE(LessThanOrEquals::load(data)),
            34 => Instruction::TEST(Test::load(data)),
            // TODO: 35 TESTSET
            36 => Instruction::CALL(Call::load(data)),
            37 => Instruction::TAILCALL(Tailcall::load(data)),
            38 => Instruction::RETURN(Return::load(data)),
            39 => Instruction::FORLOOP(ForLoop::load(data)),
            40 => Instruction::FORPREP(ForPrep::load(data)),
            // TODO: 40 FORPREP
            // TODO: 41 TFORCALL
            // TODO: 42 TFORLOOP
            // TODO: 43 SETLIST
            44 => Instruction::CLOSURE(Closure::load(data)),
            // TODO: 45 VARARG
            // TODO: 46 EXTRAARG
            invalid => panic!("invalid opcode: {:?}, all: {:?}", invalid, data)
        }
    }
}

#[allow(non_snake_case)]
pub fn parse_A_B(d: u32) -> (Reg, Reg) {
    let (a, b, _) = parse_A_B_C(d);
    (a, b)
}

#[allow(non_snake_case)]
pub fn parse_A_Bx(d: u32) -> (Reg, Reg) {
    let a = get_bits!(d, 6 => 14);
    let b = get_bits!(d, 14 => 32);
    (a as Reg, b as Reg)
}

// Field sBx can represent negative numbers, but it doesn’t use 2s complement.
// Instead, it has a bias equal to half the maximum integer that can be represented by its unsigned counterpart, Bx.
// For a field size of 18 bits, Bx can hold a maximum unsigned integer value of 262143, and so the bias is 131071 (calculated as 262143 >> 1).
// A value of -1 will be encoded as (-1 + 131071) or 131070 or 1FFFE in hexadecimal.
#[allow(non_snake_case)]
pub fn parse_A_sBx(d: u32) -> (Reg, isize) {
    let a = get_bits!(d, 6 => 14);
    let b = get_bits!(d, 14 => 32);

    let b_bits = 32 - (6 + 8);
    let b_bias = (1 << (b_bits - 1)) - 1;
    let sBx = b as isize - b_bias;
    (a as Reg, sBx)
}

#[allow(non_snake_case)]
pub fn parse_A_B_C(d: u32) -> (Reg, Reg, Reg) {
    let a = get_bits!(d, 6 => 14);
    let b = get_bits!(d, 23 => 32); // WTF!?
    let c = get_bits!(d, 14 => 23);
    (a as Reg, b as Reg, c as Reg)
}

// TODO: rename DataSource
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataSource {
    Register(usize),
    Constant(usize),
}

impl DataSource {
    pub fn get_from(&self, i: &mut Context) -> Type {
        match *self {
            DataSource::Register(index) => match i.stack[index] {
                StackEntry::Type(ref t) => t.clone(),
                _ => unimplemented!()
            },
            DataSource::Constant(index) => i.ci().func.constants[index].clone()
        }
    }
}

impl From<usize> for DataSource {
    fn from(other: usize) -> Self {
        if other >= 0b1_0000_0000 {
            DataSource::Constant(other & 0xFF)
        } else {
            DataSource::Register(other)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpvalueIndex (i64);

impl UpvalueIndex {
    pub fn load(val: usize) -> Self {
        UpvalueIndex(
            if val >= 0b1_0000_0000 {
                -(val as i64 & 0xFF)
            } else {
                val as i64
            }
        )
    }
}

impl From<usize> for UpvalueIndex {
    fn from(other: usize) -> Self {
        UpvalueIndex::load(other)
    }
}

pub struct InstructionContext<'a> {
    pub index: usize,
    pub func: &'a FunctionBlock,
    pub debug: &'a DebugData,
}

impl<'a> InstructionContext<'a> {
    pub fn filter(&self, d: Vec<Option<String>>) -> Vec<String> {
        d.iter().filter_map(|i| i.clone()).collect()
    }

    pub fn pretty_constant(&self, c: DataSource) -> Option<String> {
        (match c {
            DataSource::Constant(v) => Some(v),
            DataSource::Register(_) => None
        })
            .map(|i| (i, &self.func.constants[i]))
            .map(|(i, constant)| format!("{} = {}", i, constant.repr()))
    }

    pub fn pretty_upval(&self, u: UpvalueIndex) -> Option<String> {
        (if u.0 < 0 {
            None
        } else {
            Some(u.0 as usize)
        })
            .map(|i| &self.debug.upvalue_names[i])
            .map(|name| format!("{} = {}", u.0, name))
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Count {
    Unknown,
    Known(usize),
}

impl From<usize> for Count {
    fn from(n: usize) -> Self {
        match n {
            0 => Count::Unknown,
            _ => Count::Known(n - 1)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor; 
    use parser::Parsable;
    use instructions::*;

    #[test]
    fn get_bits_works() {
        let data = 0b1111_0001_1001_1001u16;
        assert_eq!(get_bits!(data, 0 => 6), 0b011001);
        assert_eq!(get_bits!(data, 6 => 8), 0b10);
        assert_eq!(get_bits!(data, 8 => 16), 0b1111_0001);
    }

    #[test]
    fn parses_return_instruction() {
        let data = &[0x26, 0x00, 0x80, 0x00];
        let mut reader = Cursor::new(data);
        let instruction = Instruction::parse(&mut reader);
        assert_eq!(instruction, Instruction::RETURN(Return {a: 0, b: 1}));
    }

    #[test]
    fn parses_gettabup() {
        let data = &[0b00000110, 0b00000000, 0b01000000, 0];
        let mut reader = Cursor::new(data);
        let instruction = Instruction::parse(&mut reader);
        assert_eq!(instruction, Instruction::GETTABUP(GetTabUp { reg: 0, upvalue: 0, constant: DataSource::Constant(0) }));
    }
}