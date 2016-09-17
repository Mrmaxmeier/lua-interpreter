use instruction::*;

macro_rules! logic {
    ($name:ident, $op:expr) => (
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            pub lhs: DataSource,
            pub rhs: DataSource,
            pub inverted: bool
        }

        impl LoadInstruction for $name {
            fn load(d: u32) -> Self {
                let (a, b, c) = parse_A_B_C(d);
                $name {
                    lhs: b.into(),
                    rhs: c.into(),
                    inverted: a == 0,
                }
            }
        }

        impl InstructionOps for $name {
            fn exec(&self, context: &mut Context) {
                let lhs = self.lhs.get_from(context);
                let rhs = self.rhs.get_from(context);
                let res = $op(lhs, rhs).unwrap();
                if res == self.inverted {
                    context.ci_mut().pc += 1
                }
            }
        }
    )
}

fn attempted_to_compare(a: &Type, b: &Type) -> String {
    format!("attempted to compare {} with {} ({}, {})", a.as_type_str(), b.as_type_str(), a.repr(), b.repr())
}


// 31: EQ       A B C   if ((RK(B) == RK(C)) ~= A) then pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Equals {
    pub lhs: DataSource,
    pub rhs: DataSource,
    pub inverted: bool
}

impl LoadInstruction for Equals {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Equals {
            lhs: b.into(),
            rhs: c.into(),
            inverted: a == 0,
        }
    }
}

impl InstructionOps for Equals {
    fn exec(&self, context: &mut Context) {
        let res = if let (Some(shared_lhs), Some(shared_rhs)) = (self.lhs.get_shared(context), self.rhs.get_shared(context)) {
            ::std::sync::Arc::ptr_eq(&shared_lhs, &shared_rhs)
        } else {
            let lhs = self.lhs.get_from(context);
            let rhs = self.rhs.get_from(context);
            lhs == rhs
        };
        if res == self.inverted {
            context.ci_mut().pc += 1
        }
    }
}

// 32: LT       A B C   if ((RK(B) <  RK(C)) ~= A) then pc++
logic!(LessThan, |a, b| {
    if let (&Type::Number(ref a_num), &Type::Number(ref b_num)) = (&a, &b) {
        Ok(a_num < b_num)
    } else {
        Err(attempted_to_compare(&a, &b))
    }
});

// 33: LE       A B C   if ((RK(B) <= RK(C)) ~= A) then pc++
logic!(LessThanOrEquals, |a, b| {
    if let (&Type::Number(ref a_num), &Type::Number(ref b_num)) = (&a, &b) {
        Ok(a_num <= b_num)
    } else {
        Err(attempted_to_compare(&a, &b))
    }
});
