use std::alloc::Layout;

use dlopen::wrapper::{Container, WrapperApi};

use crate::machine::Machine;

/// A Machine that is created from a C dynamic library.
pub struct CMachine {
    ptr: *mut (),
    layout: Layout,
    lib: Container<MachineApi>,
}

impl CMachine {
    /// Creates a new machine by loading the shared library at 
    pub fn new(file: &str) -> CMachine {
        let lib: Container<MachineApi> = unsafe { Container::load(file) }.unwrap();
        let size = lib.machine_size();
        let align = size.next_power_of_two();
        assert!(size <= align, "tried to allocate machine too large");

        let layout = Layout::from_size_align(size as usize, align as usize).unwrap();

        let ptr;
        unsafe {
            ptr = std::alloc::alloc(layout) as *mut ();
            assert!(!ptr.is_null());
            lib.allocate_machine(ptr);
        }

        CMachine { ptr, layout, lib }
    }
}

impl Machine for CMachine {
    fn send(&mut self, msg: u8) {
        unsafe {
            self.lib.send(self.ptr, msg);
        }
    }

    fn recv(&mut self) -> u8 {
        unsafe { self.lib.recv(self.ptr) }
    }
}

impl Drop for CMachine {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

/// Struct containing functions from a dynamic library.
/// Uses the dlopen crate's WrapperApi trait.
#[derive(WrapperApi, Clone)]
struct MachineApi {
    machine_size: extern "C" fn() -> u32,
    allocate_machine: unsafe extern "C" fn(buf: *mut ()),
    /// Function used to send a message to the machine.
    send: unsafe extern "C" fn(machine: *mut (), msg: u8),
    /// Function used to receive a message from the machine.
    recv: unsafe extern "C" fn(machine: *mut ()) -> u8,
}
