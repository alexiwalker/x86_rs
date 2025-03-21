#![allow(unused, dead_code)]
//todo remove global allow after initial development

/// Intrinsics module
///
/// This is used to separate the implementation of various system level traps, interrupts, and syscalls
/// from the implementation of the ISA operations and such
///
/// It depends on the types provided in the main lib, and the machine needs to know how to call the
/// intrinsics in the X86Machine impl, so we cannot define those types here
///
// local reexports to make the names easier
pub(crate) mod types {
    pub use lib_x86::functions::InterruptVector;
    pub use lib_x86::functions::Intrinsic;
    pub use lib_x86::functions::IntrinsicPtr;
    pub use lib_x86::functions::SyscallVector;
    pub use lib_x86::functions::SystemFunction;
    pub use lib_x86::x86::X86Machine;
}

pub(crate) mod private {
    pub trait Sealed {}
}
