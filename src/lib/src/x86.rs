use lib_types::error::{VmBuildError, VmRuntimeError};
use lib_types::memory::ByteUnits;
use crate::functions::{InterruptVector, SyscallVector, SystemFunction};
use crate::memory::{ContiguousMemory, Fpu};
use crate::registers::{RegisterWidth, Registers};

/// Represents a virtual x86_64 lib
///
/// Can be constructed with variable amounts of memory. All memory is allocated contiguously at construction of the machine
///
/// https://cs.lmu.edu/~ray/notes/x86overview/ for reference on registers, address space ETC
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct X86Machine {
    /// General purpose registers
    pub(crate) segment_pointers: Registers<{ (8*64) / 8 }>, /* 6 x 64 registers,  represented by u8s. 6 are specified, 2 extras added for padding to a pow2 */
    pub(crate) mmx_registers: Registers<{ (8*64) / 8 }>, /*  8 x 64 registers, represented by u8s */
    pub(crate) gp_registers: Registers<{ (16*64) / 8 }>, /* 16 x 64 registers, represented by u8s */
    pub(crate) xmm_registers: Registers<{ (16*128) / 8 }>, /* 16 x 128 registers, represented by u8s */
    pub(crate) ymm_registers: Registers<{ (16*256) / 8 }>, /* 16 x 256 registers, represented by u8s */
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

    pub stack: ContiguousMemory,
    pub memory: ContiguousMemory,
}

impl X86Machine {
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
}

/// An initialised set of X86Machine constructor options
///
pub struct MachineOptions {
    pub memory: ByteUnits,
    pub syscalls: SyscallVector,
    pub interrupts: InterruptVector,
}

impl MachineOptions {
    pub fn builder() -> MachineBuilder {
        MachineBuilder {
            memory: None,
            syscalls: None,
            interrupts: None,
        }
    }

    pub fn memory(mut self, memory: ByteUnits) -> Self {
        self.memory = memory;
        self
    }
    pub fn syscalls(mut self, syscalls: SyscallVector) -> Self {
        self.syscalls = syscalls;
        self
    }

    pub fn interrupts(mut self, interrupts: InterruptVector) -> Self {
        self.interrupts = interrupts;
        self
    }

    pub fn build(self) -> X86Machine {
        X86Machine {
            mmx_registers: Default::default(),
            segment_pointers: Default::default(),
            gp_registers: Default::default(),
            xmm_registers: Default::default(),
            ymm_registers: Default::default(),
            bounds_registers: Default::default(),
            mxcsr_register: Default::default(),
            fpu: Default::default(),
            flags: 0,
            instruction_counter: 0,
            stack_pointer: 0,
            interrupts: self.interrupts,
            syscalls: self.syscalls,
            stack: ContiguousMemory::with_size(&ByteUnits::GibiBytes(1)),
            memory: ContiguousMemory::with_size(&self.memory),
            assigned_memory: self.memory,
        }
    }
}

pub struct MachineBuilder {
    pub memory: Option<ByteUnits>,
    pub syscalls: Option<SyscallVector>,
    pub interrupts: Option<InterruptVector>,
}

impl MachineBuilder {

    pub fn new() -> MachineBuilder {
        MachineBuilder {
            memory: None,
            syscalls: None,
            interrupts: None,
        }
    }


    pub fn build_with_defaults(self) -> X86Machine {
        let memory = self.memory.unwrap_or(ByteUnits::GibiBytes(1));
        let syscalls = self.syscalls.unwrap_or(empty_syscalls());
        let interrupts = self.interrupts.unwrap_or(empty_interrupts());

        MachineOptions {
            memory,
            syscalls,
            interrupts,
        }
        .build()
    }

    /// Initialises the build options. Panics if any fields aren't set. Ensure they are set by
    /// creating the options with new_with_defaults(), or call build_options_with_defaults() instead
    pub fn build(self) -> X86Machine {
        MachineOptions {
            memory: self.memory.unwrap(),
            syscalls: self.syscalls.unwrap(),
            interrupts: self.interrupts.unwrap(),
        }
        .build()
    }

    pub fn try_build(self) -> Result<X86Machine, VmBuildError> {
        let mut error = VmBuildError {
            missing_memory: false,
            missing_registers: false,
            missing_interrupts: false,
        };
        let mut e = false;

        if self.memory.is_none() {
            e = true;
            error.missing_memory = true;
        }

        if self.syscalls.is_none() {
            e = true;
            error.missing_registers = true;
        }

        if self.interrupts.is_none() {
            e = true;
            error.missing_interrupts = true;
        }

        let memory = self.memory.unwrap();
        let syscalls = self.syscalls.unwrap();
        let interrupts = self.interrupts.unwrap();

        if e {
            Err(error)
        } else {
            Ok(MachineOptions {
                memory,
                syscalls,
                interrupts,
            }
            .build())
        }
    }

    pub fn build_machine(self) -> X86Machine {
        let memory = self.memory.unwrap_or(ByteUnits::GibiBytes(1));
        let syscalls = self.syscalls.unwrap_or(empty_syscalls());
        let interrupts = self.interrupts.unwrap_or(empty_interrupts());

        let machine = MachineOptions {
            memory,
            syscalls,
            interrupts,
        }
        .build();

        machine
    }

    pub fn memory(mut self, memory: ByteUnits) -> Self {
        self.memory = Some(memory);
        self
    }
    pub fn syscalls(mut self, syscalls: SyscallVector) -> Self {
        self.syscalls = Some(syscalls);
        self
    }

    pub fn interrupts(mut self, interrupts: InterruptVector) -> Self {
        self.interrupts = Some(interrupts);
        self
    }
}

fn empty_syscalls() -> SyscallVector {
    SyscallVector([SystemFunction::default(); 1024])
}

fn empty_interrupts() -> InterruptVector {
    InterruptVector([SystemFunction::default(); 255])
}


// 1024 should be enough
