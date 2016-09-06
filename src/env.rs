use types::{Type, LuaTable, SharedType};
use function::{Function, NativeFunction};
use std::sync::Arc;
use parking_lot::Mutex;
use std::sync::mpsc;

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
        ("type", Box::new(
            |ref mut i| {
                let output: Type = {
                    let se = i.get(0);
                    se.as_type().as_type_str().into()
                };
                i.returns(output);
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
    fn insert_standard(table: &mut LuaTable) {
        Self::insert_funcs(table, standard_functions());
        table.insert("_VERSION".into(), "Lua 5.3".into());
    }

    fn insert_funcs(table: &mut LuaTable, funcs: Vec<(&'static str, NativeFunction)>) {
        for (name, func) in funcs {
            let as_func: Function = func.into();
            table.insert(name.into(), as_func.into());
        }
    }

    pub fn make(&self) -> SharedType {
        let mut table = LuaTable::new();
        match *self {
            Environment::Empty => {},
            Environment::LuaStandard => Self::insert_standard(&mut table),
            Environment::Testing(ref tx) => {
                Self::insert_standard(&mut table);
                Self::insert_funcs(&mut table, testing_funcs(tx.clone()));
            },
        }
        let as_type = Type::Table(table);
        Arc::new(Mutex::new(as_type))
    }
}
