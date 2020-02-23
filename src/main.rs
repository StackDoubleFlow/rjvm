mod class;
mod storage_manager;
mod thread;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate human_panic;

use clap::App;

// https://docs.oracle.com/javase/specs/jvms/se13/html/

fn main() {
    setup_panic!();
    App::new("rjvm")
        .version("0.1.0")
        .about("A Java Virtual Machine completely written in rust.")
        .author("StackDoubleFlow <ojaslandge@gmail.com>")
        .get_matches();
}

#[test]
fn test() {
    println!("Hello, world!");
}
