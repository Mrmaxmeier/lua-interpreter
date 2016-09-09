use instruction::*;

macro_rules! logic {
    ($name:ident, $op:expr) => (
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            pub a: DataSource,
            pub b: DataSource,
            pub inverted: bool
        }

        impl LoadInstruction for $name {
            fn load(d: u32) -> Self {
                let (a, b, c) = parse_A_B_C(d);
                $name {
                    a: a.into(),
                    b: b.into(),
                    inverted: c == 0,
                }
            }
        }

        impl InstructionOps for $name {
            fn exec(&self, closure: &mut ClosureCtx) {
                let a = self.a.get_from(closure);
                let b = self.b.get_from(closure);
                let eq = $op(a, b).unwrap();
                if eq ^ self.inverted {
                    closure.pc.skip(1)
                }
            }
        }
    )
}

fn attempted_to_compare(a: &Type, b: &Type) -> String {
    format!("attempted to compare {} with {} ({}, {})", a.as_type_str(), b.as_type_str(), a.repr(), b.repr())
}


// 31: EQ       A B C   if ((RK(B) == RK(C)) ~= A) then pc++
logic!(Equals, |a, b| -> Result<bool, String> {
    Ok(a == b)
});
// 32: LT       A B C   if ((RK(B) <  RK(C)) ~= A) then pc++
logic!(LessThan, |a, b| {
    match a {
        Type::Number(ref a_num) => {
            match b {
                Type::Number(ref b_num) => Ok(a_num < b_num),
                b => Err(attempted_to_compare(&a, &b)) 
            }
        },
        a => Err(attempted_to_compare(&a, &b)) 
    }
});

// 33: LE       A B C   if ((RK(B) <= RK(C)) ~= A) then pc++
logic!(LessThanOrEquals, |a, b| {
    match a {
        Type::Number(ref a_num) => {
            match b {
                Type::Number(ref b_num) => Ok(a_num <= b_num),
                b => Err(attempted_to_compare(&a, &b)) 
            }
        },
        a => Err(attempted_to_compare(&a, &b)) 
    }
});
