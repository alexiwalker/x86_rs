
#[derive(Debug, Clone)]
pub enum VmRuntimeError {
    FdReadError { code: u32, message: String },
    FdWriteError { code: u32, message: String },


    OutOfMemoryError { allocated:u64, required:u64 },

    OutOfBoundsError { address: u64 },

    InterruptNotFound { code: u32 },

    SyscallNotFound { code: u32 },

    // Invalid aliases should only occur during development / testing, don't include in default builds
    InvalidAlias {offset: u8, width: u8 },

    RegisterAlasOverrun {offset: u8, width: u8, alignment: u8 }
}


#[cfg(feature = "safety_checks")]
pub type SafetyResult<T> = Result<T,VmRuntimeError>;

#[cfg(not(feature = "safety_checks"))]
pub type SafetyResult<T> = T;

#[derive(Debug, Clone, Copy,)]
pub struct VmBuildError {
    pub missing_memory: bool,
    pub missing_registers: bool,
    pub missing_interrupts: bool,
}