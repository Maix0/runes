#![allow(clippy::wildcard_in_or_patterns)]
#[macro_use]
extern crate serde;
extern crate arrayvec;
extern crate bitflags;
extern crate proc_macro2;
extern crate quote;
extern crate scraper;
extern crate serde_json;
extern crate syn;
extern crate walkdir;
mod builder;
mod opcode;

fn main() {
    let mut args = std::env::args().skip(1);
    let task = args.next().unwrap_or_else(|| "help".to_string());
    match task.as_str() {
        "opcode" => {
            opcode::run();
        }
        "build" => {
            let opcodes = opcode::run();
            builder::run(opcodes);
        }
        "help" | _ => print!(
            r"
xtask:
    opcode: build the opcodes data
    build:  build the rust code for the opcode data
    help:   show this message
"
        ),
    }
}
