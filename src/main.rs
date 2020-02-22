mod class;
mod storage_manager;
mod thread;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate human_panic;

// https://docs.oracle.com/javase/specs/jvms/se13/html/

fn main() {
    setup_panic!();
}

#[test]
fn test() {
    println!("Hello, world!");
}
