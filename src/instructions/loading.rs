use instruction::*;

// 00: MOVE   A B   R(A) := R(B)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { pub to: Reg, pub from: Reg }

impl LoadInstruction for Move {
    fn load(d: u32) -> Self {
        let (to, from) = parse_A_B(d);
        Move {
            to: to,
            from: from,
        }
    }
}

impl InstructionOps for Move {
    fn exec(&self, context: &mut Context) {
        let val = context.stack[self.from].clone();
        context.stack[self.to] = val;
    }
}

// 01: LOADK   A Bx   R(A) := Kst(Bx)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadK { pub local: Reg, pub constant: Reg }

impl LoadInstruction for LoadK {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_Bx(d);
        LoadK {
            local: a,
            constant: b,
        }
    }
}

impl InstructionOps for LoadK {
    fn exec(&self, context: &mut Context) {
        let c = context.ci().func.constants[self.constant].clone();
        context.stack[self.local] = c.into();
    }

    fn debug_info(&self, c: InstructionContext) -> Vec<String> {
        c.filter(vec![
            c.debug.locals
                .get(self.local as usize)
                .map(|local| {
                    let local_in_range = local.startpc <= c.index as u32 && local.endpc >= c.index as u32;
                    if local_in_range {
                        format!("{} = {}", self.local, local)
                    } else {
                        format!("{}", self.local)
                    }
                }),
            c.pretty_constant(DataSource::Constant(self.constant)),
        ])
    }
}


// 03: LOADBOOL     A B C       R(A) := (Bool)B; if (C) pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadBool { pub reg: Reg, pub value: bool, pub jump: bool }

impl LoadInstruction for LoadBool {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        LoadBool {
            reg: a,
            value: b > 0,
            jump: c > 0,
        }
    }
}

impl InstructionOps for LoadBool {
    fn exec(&self, context: &mut Context) {
        context.stack[self.reg] = Type::Boolean(self.value).into();
        if self.jump {
            context.ci_mut().pc += 1
        }
    }
    fn debug_info(&self, c: InstructionContext) -> Vec<String> {
        if let Some(local) = c.debug.locals.get(self.reg as usize) {
            vec![
                format!("{} = {}", self.reg, local),
            ]
        } else {
            vec![]
        }
    }
}

// 04: LOADNIL      A B         R(A), R(A+1), ..., R(A+B) := nil
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadNil { pub start: Reg, pub range: usize }

impl LoadInstruction for LoadNil {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_B(d);
        LoadNil {
            start: a,
            range: b as usize,
        }
    }
}

impl InstructionOps for LoadNil {
    fn exec(&self, context: &mut Context) {
        for i in self.start..self.start + self.range + 1 {
            context.stack[i] = (Type::Nil).into();
        }
    }
    fn debug_info(&self, c: InstructionContext) -> Vec<String> {
        let start = self.start as usize;
        let end = start + self.range + 1;
        if c.debug.locals.len() <= end {
            return vec![];
        }
        (start..end).map(|i| format!("{} = {}", i, c.debug.locals[i])).collect()
    }
}