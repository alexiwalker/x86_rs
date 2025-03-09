use std::ops::Deref;
use crate::prelude::X86Machine;

pub type IntrinsicPtr = fn(&mut X86Machine) -> ();

/// An intrinsic is a function that runs on the machine itself - it is used to implement
/// system calls and interrupts at the library level. There
#[derive(Debug, Copy, Clone)]
pub struct Intrinsic(pub fn(&mut X86Machine) -> ());

impl Intrinsic {

    #[inline(always)]
    pub fn from_ptr(f: fn(&mut X86Machine)) -> Intrinsic {
        Intrinsic(f)
    }
}

impl Deref for Intrinsic {
    type Target = fn(&mut X86Machine) -> ();

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Allows interrupts and syscalls to be implemented either in loaded binary (Pointer) or
/// at machine compile time by intrinsics
///
#[derive(Debug, Default, Copy, Clone)]
pub enum SystemFunction {
    /// No such function exists. Will cause a vm panic if called
    #[default]
    Unimplemented,

    /// Compile-time function
    IntrinsicFunction(Intrinsic),

    /// Function loaded from binary and placed into memory
    Pointer(u64),
}

impl SystemFunction {
    pub fn call(&self, machine: &mut X86Machine) -> () {
        match self {
            SystemFunction::Unimplemented => {}
            SystemFunction::IntrinsicFunction(ptr) => {
                (*ptr)(machine);
            }
            SystemFunction::Pointer(ptr) => {
                machine.set_instruction_counter(*ptr)
                // machine.resume()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InterruptVector(pub(crate) [SystemFunction; 255]);

#[derive(Debug, Clone, Copy)]
pub struct SyscallVector(pub(crate) [SystemFunction; 1024]);