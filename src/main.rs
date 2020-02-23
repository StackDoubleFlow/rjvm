mod class;
mod storage_manager;
mod thread;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate human_panic;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rjvm", version = "0.1.0", author = "StackDoubleFlow <ojaslandge@gmail.com>", about = "A Java Virtual Machine completely written in rust.")]
struct Opts {

    /// Path to the main class file
    #[structopt(parse(from_os_str))]
    class_file: PathBuf
}

fn main() {
    setup_panic!();
    let opts = Opts::from_args();
    let data = std::fs::read(opts.class_file).unwrap();
    println!("{:#?}", class::Class::deserialize(data));
}

#[test]
fn test() {
    println!("Hello, world!");
}
