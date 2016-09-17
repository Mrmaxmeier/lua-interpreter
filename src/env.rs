use types::Type;
use table::LuaTableRaw;
use function::{Function, NativeFunction};
use std::sync::mpsc;
use std::collections::BTreeMap;

fn standard_functions() -> Vec<(&'static str, NativeFunction)> {
    vec![
        ("print", Box::new(
            |ref mut i| {
                let args = i.arguments();
                let output: Vec<_> = args.iter()
                    .map(|t| format!("{}", t.as_type()))
                    .collect();
                println!("{}", output.join("\t"));
            }
        )),
        ("assert", Box::new(
            |ref mut i| {
                let arg = i.get(0).as_type();
                let will_panic = match arg {
                    Type::Nil => true,
                    Type::Boolean(v) => !v,
                    _ => false 
                };
                if will_panic {
                    panic!("assertion failed {:?}", arg);
                }
                let args: Vec<_> = i.arguments()
                    .iter()
                    .map(|a| a.as_type())
                    .collect();
                i.returns(args);
            }
        )),
        ("type", Box::new(
            |ref mut i| {
                let output: Type = {
                    let se = i.get(0);
                    se.as_type().as_type_str().into()
                };
                i.returns(vec![output]);
            }
        ))
    ]
}

fn testing_funcs(tx: mpsc::Sender<String>) -> Vec<(&'static str, NativeFunction)> {
    vec![
        ("print", Box::new(
            move |ref mut i| {
                let args = i.arguments();
                let output: Vec<_> = args.iter()
                    .map(|t| format!("{}", t.as_type()))
                    .collect();
                tx.send(output.join("\t")).unwrap()
            }
        )),
    ]
}

pub enum Environment {
    Empty,
    LuaStandard, // lbaselib.c 453 - 483
    Testing(mpsc::Sender<String>),
}

impl Environment {
    fn insert_standard(table: &mut LuaTableRaw) {
        Self::insert_funcs(table, standard_functions());
        table.insert("_VERSION".into(), "Lua 5.3".into());
    }

    fn insert_funcs(table: &mut LuaTableRaw, funcs: Vec<(&'static str, NativeFunction)>) {
        for (name, func) in funcs {
            let as_func: Function = func.into();
            table.insert(name.into(), as_func.into());
        }
    }

    pub fn make(&self) -> Type {
        let mut table: LuaTableRaw = BTreeMap::new();
        match *self {
            Environment::Empty => {},
            Environment::LuaStandard => Self::insert_standard(&mut table),
            Environment::Testing(ref tx) => {
                Self::insert_standard(&mut table);
                Self::insert_funcs(&mut table, testing_funcs(tx.clone()));
            },
        }
        Type::Table(table.into())
    }
}
