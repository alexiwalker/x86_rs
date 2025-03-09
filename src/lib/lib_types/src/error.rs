
#[derive(Debug, Clone)]
pub enum VmRuntimeError {
    FdReadError { code: u32, message: String },
    FdWriteError { code: u32, message: String },


    OutOfMemoryError { allocated:u64, required:u64 },

    OutOfBoundsError { address: u64 },

    InterruptNotFound { code: u32 },

    SyscallNotFound { code: u32 },

}

#[derive(Debug, Clone, Copy,)]
pub struct VmBuildError {
    pub missing_memory: bool,
    pub missing_registers: bool,
    pub missing_interrupts: bool,
}