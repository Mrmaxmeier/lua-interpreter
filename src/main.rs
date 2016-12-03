extern crate lua_interpreter;
use lua_interpreter::interpreter::Interpreter;
use lua_interpreter::env::Environment;
use lua_interpreter::bytecode::Bytecode;
use lua_interpreter::parser::Parsable;

use std::fs::File;
use std::io::Cursor;
use std::process::Command;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("lua-interpreter")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .arg(Arg::with_name("debug")
                               .short("d")
                               .help("Runs and displays debug info"))
                          .arg(Arg::with_name("no-debug")
                               .short("n")
                               .help("Runs without displaying debug info"))
                          .arg(Arg::with_name("prettyprint")
                               .short("p")
                               .help("Prettyprints bytecode data"))
                          .get_matches();

    let mut file_path = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", file_path);

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    if file_path.ends_with(".lua") {
        let compile_output = Command::new("luac")
            .arg("-o")
            .arg("/tmp/luac.out")
            .arg(file_path)
            .output()
            .unwrap();
        println!("{:?}", compile_output);
        file_path = "/tmp/luac.out";
    }

    let mut f = File::open(file_path).unwrap();
    let bytecode = Bytecode::parse(&mut f);

    if matches.is_present("prettyprint") {
        let mut stream = Cursor::new(Vec::new());
        bytecode.pretty_print(&mut stream).unwrap();
        let pprint_result: String = String::from_utf8(stream.into_inner()).unwrap();
        println!("{}", pprint_result);
    }

    let mut interpreter = Interpreter::new(bytecode, Environment::LuaStandard);

    if matches.is_present("debug") {
        interpreter.run_debug();
    }
    if matches.is_present("no-debug") {
        interpreter.run();
    }
}
