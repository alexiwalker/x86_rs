use lib_types::error::{VmBuildError, VmRuntimeError};
use lib_types::memory::ByteUnits;
use std::fmt;
use std::ops::Deref;
use crate::functions::{InterruptVector, SyscallVector, SystemFunction};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SegmentPointers {
    stack: u64,
    code: u64,
    data: u64,

    extra: u64,   // aka E
    extra_2: u64, // aka F (F comes after E
    extra_3: u64, // aka G (G comes after F)

    unused_1: u64, // unused, basically padding alignment (to get to 8 x 64 )
    unused_2: u64, // unused, basically padding alignment (to get to 8 x 64 )
}

#[derive(Clone)]
// equivalent to 16 x 64 bit registers
// created as u8 here to make addressing specific segments and aliases easier
pub(crate) struct Registers<const N: usize>(pub(crate) [u8; N]);


impl<const N: usize> Deref for Registers<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub(crate) struct ContiguousMemory(pub(crate) Vec<u8>);

impl Deref for ContiguousMemory {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for Registers<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::truncate_hex(f, &self.0)
    }
}

impl fmt::Debug for ContiguousMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::truncate_hex(f, &self.0)
    }
}

impl<const N: usize> fmt::Display for Registers<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::truncate_hex(f, &self.0)
    }
}

impl fmt::Display for ContiguousMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::truncate_hex(f, &self.0)
    }
}

pub(crate) struct Fpu {
    pub(crate) registers:  Registers<80> /* 8 x 80 bit registers, represented by u8s*/
}

/// Represents a virtual x86_64 lib
///
/// Can be constructed with variable amounts of memory. All memory is allocated contiguously at construction of the machine
///
/// https://cs.lmu.edu/~ray/notes/x86overview/ for reference on registers, address space ETC
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct X86Machine {
    /// General purpose registers
    pub(crate) gp_registers: Registers<{ (16*64) / 8 }>, /* 16 x 64 registers, represented by u8s */
    pub(crate) mmx_registers: Registers<{ (8*64) / 8 }>, /*  8 x 64 registers, represented by u8s */
    pub(crate) xmm_registers: Registers<{ (16*128) / 8 }>, /* 16 x 128 registers, represented by u8s */
    pub(crate) mxcsr_register: Registers<{ 32 / 8 }>, /* 1 x 32 register, represented by u8s */

    pub(crate) bounds_registers: Registers<{ (4 * 128) / 8 }>, /* 4 x 128 registers, represented by u8s. Aliases: upper = BNDCFGU, lower = BNDSTATUS */

    pub(crate) ymm_registers: Registers<{ (16*256) / 8 }>, /* 16 x 256 registers, represented by u8s */

    pub(crate) segment_pointers: SegmentPointers,

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

impl ContiguousMemory {
    pub fn with_size(size: &ByteUnits) -> Self {
        Self(vec![0; size.num_bytes() as usize])
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
            gp_registers: Registers([0; 128]),
            mmx_registers: Registers([0; 64]),
            xmm_registers: Registers([0; 256]),
            mxcsr_register: Registers([0; 4]),
            bounds_registers: Registers([0; 64]),
            ymm_registers: Registers([0; 512]),
            segment_pointers: SegmentPointers {
                stack: 0,
                code: 0,
                data: 0,
                extra: 0,
                extra_2: 0,
                extra_3: 0,
                unused_1: 0,
                unused_2: 0,
            },
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
