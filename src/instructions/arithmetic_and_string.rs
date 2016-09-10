use instruction::*;
use types::Number;

macro_rules! arith {
    ($name:ident, $op:expr) => (
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            pub a: Reg,
            pub b: DataSource,
            pub c: DataSource
        }

        impl LoadInstruction for $name {
            fn load(d: u32) -> Self {
                let (a, b, c) = parse_A_B_C(d);
                $name {
                    a: a,
                    b: b.into(),
                    c: c.into()
                }
            }
        }

        impl InstructionOps for $name {
            fn exec(&self, closure: &mut ClosureCtx) {
                let b = self.b.get_from(closure);
                let c = self.c.get_from(closure);
                if let (&Type::Number(ref b), &Type::Number(ref c)) = (&b, &c) {
                    let result = $op(*b, *c);
                    closure.stack[self.a] = StackEntry::Type(Type::Number(result));
                } else {
                    panic!("invalid types, expected numbers ({}, {})", b.as_type_str(), c.as_type_str())
                }
            }
        }
    )
}

macro_rules! warp_as_number_type {
    ($f:expr) => (|a, b| {
        match a {
            Number::Integer(a) => {
                match b {
                    Number::Integer(b) => Number::Integer($f(a, b)),
                    Number::Float(b) => Number::Float($f(a as f64, b))
                }
            },
            Number::Float(a) => {
                match b {
                    Number::Integer(b) => Number::Float($f(a, b as f64)),
                    Number::Float(b) => Number::Float($f(a, b))
                }
            }
        }
    })
}

macro_rules! as_integer_repr {
    ($f:expr) => (|a, b| {
        if let (&Number::Integer(ref a), &Number::Integer(ref b)) = (&a, &b) {
            Number::Integer($f(*a, *b))
        } else {
            panic!("number has no integer representation ({:?}, {:?})", a, b)
        }
    })
}

macro_rules! wrapped_type_as_integer {
    ($f:expr) => (|val| {
        if let Type::Number(Number::Integer(v)) = val {
            Type::Number(Number::Integer($f(v)))
        } else {
            panic!("{} has no integer representation", val.as_type_str())
        }
    })
}


// ADD,         A B C   R(A) := RK(B) + RK(C)                           13
arith!(Add, warp_as_number_type!(|a, b| a + b));
// SUB,         A B C   R(A) := RK(B) - RK(C)                           14
arith!(Sub, warp_as_number_type!(|a, b| a - b));
// MUL,         A B C   R(A) := RK(B) * RK(C)                           15
arith!(Mul, warp_as_number_type!(|a, b| a * b));
// MOD,         A B C   R(A) := RK(B) % RK(C)                           16
arith!(Mod, warp_as_number_type!(|a, b| a % b));
// POW,         A B C   R(A) := RK(B) ^ RK(C)                           17
arith!(Pow, |a, b| {
    match a {
        Number::Integer(a) => {
            match b {
                Number::Integer(b) => Number::Integer(a.pow(b as u32)),
                Number::Float(b) => Number::Float((a as f64).powf(b)),
            }
        },
        Number::Float(a) => {
            match b {
                Number::Integer(b) => Number::Float(a.powi(b as i32)),
                Number::Float(b) => Number::Float(a.powf(b))
            }
        }
    }
});
// DIV,         A B C   R(A) := RK(B) / RK(C)                           18
arith!(Div, warp_as_number_type!(|a, b| a / b));
// IDIV,        A B C   R(A) := RK(B) // RK(C)                          19
arith!(IDiv, |a: Number, b: Number| {
    let a: f64 = a.into();
    let b: f64 = b.into();
    Number::Integer((a / b) as i64)
});

// BAND,        A B C   R(A) := RK(B) & RK(C)                           20
arith!(BAnd, as_integer_repr!(|a, b| a & b));
// BOR,         A B C   R(A) := RK(B) | RK(C)                           21
arith!(BOr, as_integer_repr!(|a, b| a | b));
// BXOR,        A B C   R(A) := RK(B) ~ RK(C)                           22
arith!(BXor, as_integer_repr!(|a, b| a ^ b));
// SHL,         A B C   R(A) := RK(B) << RK(C)                          23
arith!(Shl, as_integer_repr!(|a, b| a << b));
// SHR,         A B C   R(A) := RK(B) >> RK(C)                          24
arith!(Shr, as_integer_repr!(|a, b| a >> b));



macro_rules! unary {
    ($name:ident, $op:expr) => (
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            pub a: Reg,
            pub b: Reg,
        }

        impl LoadInstruction for $name {
            fn load(d: u32) -> Self {
                let (a, b) = parse_A_B(d);
                $name {
                    a: a,
                    b: b,
                }
            }
        }

        impl InstructionOps for $name {
            fn exec(&self, closure: &mut ClosureCtx) {
                let value = closure.stack[self.b].as_type();
                let result = $op(value);
                closure.stack[self.a] = StackEntry::Type(result);
            }
        }
    )
}

// UNM,         A B     R(A) := -R(B)                                   25
unary!(Unm, wrapped_type_as_integer!(|value: i64| -value));
// BNOT,        A B     R(A) := ~R(B)                                   26
unary!(BNot, wrapped_type_as_integer!(|value: i64| !value));
// NOT,         A B     R(A) := not R(B)                                27
unary!(Not, |value: Type| Type::Boolean(match value {
    Type::Boolean(v) => !v,
    _ => false
}));
// LEN,         A B     R(A) := length of R(B)                          28
unary!(Len, |value: Type| Type::Number(Number::Integer(match value {
    Type::String(s) => s.len() as i64,
    Type::Table(_) => unimplemented!(),
    _ => panic!("attempt to get length of a {} value ({})", value.as_type_str(), value.repr())
})));

// 29: CONCAT   A B C   R(A) := R(B).. ... ..R(C)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Concat { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Concat {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Concat {
            a: a,
            b: b,
            c: c,
        }
    }
}