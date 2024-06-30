use machine::Machine;

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;

mod c;
mod machine;

const LIB_LOCATION: &str = "./target/debug/libplugin.so";

fn main() {
    println!("current directory: {:?}", std::env::current_dir());
    let mut machine = c::CMachine::new(LIB_LOCATION);

    machine.send(5 as u8);
    println!("machine sent back the message: {}", machine.recv());
}
