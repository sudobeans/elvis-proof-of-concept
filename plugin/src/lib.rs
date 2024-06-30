#![crate_type = "dylib"]
#![allow(unused_variables)]

struct EchoMachine(u8);

#[no_mangle]
pub extern "C" fn machine_size() -> u32 {
    std::mem::size_of::<EchoMachine>()
        .try_into()
        .expect("machine size should be less than u32::MAX")
}

#[no_mangle]
pub unsafe extern "C" fn allocate_machine(buf: *mut ()) {
    *(buf as *mut EchoMachine) = EchoMachine(0);
}

#[no_mangle]
pub unsafe extern "C" fn send(machine: *mut (), msg: u8) {
    let machine: &mut EchoMachine = &mut *machine.cast::<EchoMachine>();
    machine.0 = msg;
}

#[no_mangle]
pub unsafe extern "C" fn recv(machine: *mut ()) -> u8 {
    let machine: &mut EchoMachine = &mut *machine.cast::<EchoMachine>();
    machine.0
}

pub fn main() {
    println!("Skbidi ohio");
}
