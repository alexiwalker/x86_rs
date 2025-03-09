
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
    pub use lib::x86::{InterruptVector, Intrinsic, SyscallVector, SystemFunction, X86Machine, IntrinsicPtr};
}


pub(crate) mod private {
    pub trait Sealed {}
}




#[cfg(test)]
mod tests {

    use super::*;
    fn test_intrinsic(machine: &mut types::X86Machine) -> () {
        println!("test_intrinsic. machine has bytes: {}", machine.assigned_memory.num_bytes());
    }

    use lib::prelude::{ByteUnits, Intrinsic};

    #[test]
    fn can_convert_function() {

        let mut machine = types::X86Machine::builder()
            .memory(ByteUnits::KibiBytes(512))
            .build_with_defaults();

        let intrinsic = Intrinsic::from_ptr(test_intrinsic);

        test_intrinsic(&mut machine);

    }

}