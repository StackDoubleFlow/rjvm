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
#[structopt(
    name = "rjvm",
    version = "0.1.0",
    author = "StackDoubleFlow <ojaslandge@gmail.com>",
    about = "A Java virtual machine completely written in rust."
)]
struct Opts {
    /// Path to the main class file
    #[structopt(parse(from_os_str))]
    initial_class: PathBuf,
}

fn main() {
    //setup_panic!();
    let opts = Opts::from_args();

    run(opts.initial_class);
}

fn run(initial_class: PathBuf) {
    let mut storage = storage_manager::Storage::new();
    storage.create_class(initial_class.to_str().unwrap());
}

#[test]
fn test() {
    println!("Hello, world!");
}
