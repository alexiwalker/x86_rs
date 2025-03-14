use lib_types::error::VmBuildError;
use lib_types::memory::ByteUnits;
use crate::functions::{InterruptVector, SyscallVector, SystemFunction};
use crate::memory::ContiguousMemory;
use crate::prelude::X86Machine;

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
        let mem = ContiguousMemory::with_size(&self.memory);

        let sp = mem.len();

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
            stack_pointer: sp as u64,
            interrupts: self.interrupts,
            syscalls: self.syscalls,
            // stack: ContiguousMemory::with_size(&ByteUnits::GibiBytes(1)),
            memory: mem,
            assigned_memory: self.memory,
        }
    }

}

#[derive(Default)]
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
        MachineOptions {
            memory,
            syscalls,
            interrupts,
        }
            .build()
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