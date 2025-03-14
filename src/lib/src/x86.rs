use std::ops::DerefMut;
use crate::functions::{InterruptVector, SyscallVector};
use crate::memory::{ContiguousMemory, Fpu};
use crate::registers::Registers;
use lib_types::error::{SafetyResult, VmRuntimeError};
use lib_types::memory::ByteUnits;
use crate::builders::MachineBuilder;
use crate::register_aliases::Alias;
use crate::x86::dto::MemWriteDto;

/// Represents a virtual x86_64 lib
///
/// Can be constructed with variable amounts of memory. All memory is allocated contiguously at construction of the machine
///
/// https://cs.lmu.edu/~ray/notes/x86overview/ for reference on registers, address space ETC
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct X86Machine {
    /// General purpose registers
    pub(crate) segment_pointers: Registers<{ (8 * 64) / 8 }>, /* 6 x 64 registers,  represented by u8s. 6 are specified, 2 extras added for padding to a pow2 */
    pub(crate) mmx_registers: Registers<{ (8 * 64) / 8 }>, /*  8 x 64 registers, represented by u8s */
    pub(crate) gp_registers: Registers<{ (16 * 64) / 8 }>, /* 16 x 64 registers, represented by u8s */
    pub(crate) xmm_registers: Registers<{ (16 * 128) / 8 }>, /* 16 x 128 registers, represented by u8s */
    pub(crate) ymm_registers: Registers<{ (16 * 256) / 8 }>, /* 16 x 256 registers, represented by u8s */
    pub(crate) bounds_registers: Registers<{ (4 * 128) / 8 }>, /* 4 x 128 registers, represented by u8s. Aliases: upper = BNDCFGU, lower = BNDSTATUS */
    pub(crate) mxcsr_register: Registers<{ 32 / 8 }>, /* 1 x 32 register, represented by u8s */

    pub(crate) fpu: Fpu,

    /// Bitfield flag, aka RFLAGS, EFLAGS
    pub flags: u64,

    /// AKA RIP
    pub instruction_counter: u64,

    pub stack_pointer: u64,

    /// registers are represented as contiguous memory instead of u32/64s
    /// because some operations act on segments of a particular register
    /// eg Rax / Eax / Ax / Ah / Al all being regions of the same register
    pub interrupts: InterruptVector,

    pub syscalls: SyscallVector,

    pub assigned_memory: ByteUnits,

    // pub stack: ContiguousMemory,
    pub memory: ContiguousMemory,
}

impl X86Machine {

    pub fn dump_register_hex(&self) -> String {
        self.gp_registers.dump_hex()
    }

    pub fn write_to_gp_registers(&mut self, alias:Alias, bytes: &[u8]) {
        let _ = self.gp_registers.write_bytes(alias, bytes);
    }
    pub fn load_binary(&mut self, data: &[u8]) -> Result<(), VmRuntimeError> {
        let len = data.len() as u64;
        let assigned = self.assigned_memory.num_bytes();
        if assigned < len {
            return Err(VmRuntimeError::OutOfMemoryError {
                allocated: assigned,
                required: len,
            });
        }

        todo!()
    }

    pub fn builder() -> MachineBuilder {
        MachineBuilder::new()
    }

    pub fn set_instruction_counter(&mut self, ptr: u64) {
        self.instruction_counter = ptr;
    }
    
    fn read_register_bytes<'a>(&self, alias:&Alias) -> SafetyResult<&[u8]> {
        self.gp_registers.read_bytes(&alias)
    }

    pub fn push_gp_register_to_stack(&mut self, register: Alias) -> () /* modifying in place for most ops*/ {

        let regs = &self.gp_registers;

        let stack_mem = &mut self.memory;

        let ptr = &mut self.stack_pointer;

        let dto = MemWriteDto {
            mem: stack_mem,
            register: regs,
            s_ptr: ptr,
        };

        Self::write_bytes_to_stack_memory(dto,&register);
        // let bytes = self.read_register_bytes(register);
        // self.write_bytes_to_stack(bytes);
    }
    //
    // pub fn push_gp_register_to_stack(&mut self, register:Alias) -> () /* modifying in place for most ops*/ {
    //     let mut bytes = { self.read_register_bytes(register) };
    //
    //     dbg!(&bytes);
    //     #[cfg(feature = "safety_checks")]
    //     {
    //
    //         self
    //             .write_bytes_to_stack(
    //                 bytes.unwrap()
    //             );
    //     }
    //     #[cfg(not(feature = "safety_checks"))]
    //     {
    //         self.write_bytes_to_stack(bytes);
    //
    //     }
    //
    //     ()
    // }

    pub fn write_bytes_to_stack(&mut self, bytes: &[u8]) -> SafetyResult<()> {
        //todo errors later

        let sp = self.stack_pointer as usize;
        let l = bytes.len();

        // sp starts at max and counts down, so eg if sp is 999,999 and we write 4 bytes
        //we go down to ...999, 9998, 9997, 9996

        let invert_start = sp-l;
        self.memory.write(invert_start,bytes);


        #[cfg(feature = "safety_checks")]
        {
            Ok(())
        }

        #[cfg(not(feature = "safety_checks"))] {

            ()
        }

    }


    pub fn write_bytes_to_stack_memory<const N : usize> (d: MemWriteDto<N>, alias:&Alias) -> SafetyResult<()> {

        let mem = d.mem;
        let register = d.register;
        let mut ptr = d.s_ptr;

        let bytes = register.read_bytes(&alias);


        #[cfg(feature = "safety_checks")]
            let bytes = bytes.unwrap();


        let l = bytes.len() as u64;

        let invert_start = *ptr-l;
        mem.write(invert_start as usize,bytes);

        *ptr -= l;


        #[cfg(feature = "safety_checks")] {
            Ok(())
        }

    }
}


mod dto {
    use crate::memory::ContiguousMemory;
    use crate::registers::Registers;

    pub struct MemWriteDto<'a,const N: usize > {
        pub mem: &'a mut ContiguousMemory,
        pub register: &'a Registers<N>,
        pub s_ptr:&'a mut u64,
    }
}
