use crate::registers::Registers;
use lib_types::error::VmRuntimeError;
use lib_types::memory::ByteUnits;
use std::fmt;
use std::ops::Deref;

#[derive(Clone)]
pub struct ContiguousMemory(Vec<u8>);

impl Deref for ContiguousMemory {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for ContiguousMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::format_truncated_hex(f, &self.0)
    }
}

impl fmt::Display for ContiguousMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::format_truncated_hex(f, &self.0)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Fpu {
    pub(crate) registers: Registers<80>, /* 8 x 80 bit registers, represented by u8s*/
    pub(crate) control_register: Registers<{ 16 / 8 }>, /* 1x16 bit registers, represented by u8s*/
    pub(crate) status_register: Registers<{ 16 / 8 }>, /* 1x16 bit registers, represented by u8s*/
    pub(crate) tag_register: Registers<{ 16 / 8 }>, /* 1x16 bit registers, represented by u8s*/
    pub(crate) opcopde_register: Registers<{ 16 / 8 }>, /* 1x11 bit registers, rounded to 16 for convenience, represented by u8s*/
    pub(crate) fpu_instruction_pointer: Registers<{ 64 / 8 }>, /* 1x64 bit registers, represented by u8s*/
    pub(crate) fpu_data_pointer: Registers<{ 64 / 8 }>, /* 1x11 bit registers, represented by u8s*/
}
impl ContiguousMemory {
    pub fn with_size(size: &ByteUnits) -> Self {
        Self(vec![0; size.num_bytes() as usize])
    }

    pub fn write(&mut self, addr: usize, data: &[u8]) -> Result<(), VmRuntimeError> {
        // todo IMPORTANT this needs tests to make sure it doesnt have off by 1 or range inclusive/ exclusive errors

        let len = data.len();
        let allocated_len = self.0.len();

        let total = addr + len;
        if total > allocated_len {
            return Err(VmRuntimeError::OutOfBoundsError {
                address: (addr + len) as u64,
            });
        }

        let _: Vec<_> = self
            .0
            .splice(addr..addr + len, data.iter().cloned())
            .collect();

        let new_len = self.0.len();

        if new_len > allocated_len {
            println!("second");
            Err(VmRuntimeError::OutOfBoundsError {
                address: (addr + len) as u64,
            })
        } else {
            Ok(())
        }
    }

    pub fn read(&self, addr: usize, len: usize) -> Result<&[u8], VmRuntimeError> {
        let max = self.0.len();

        if addr + len > max {
            return Err(VmRuntimeError::OutOfBoundsError {
                address: (addr + len) as u64,
            });
        }

        let v: &[u8] = self.0[addr..addr + len].as_ref();

        Ok(v)
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, VmRuntimeError> {
        let max = self.0.len();
        if addr > max {
            return Err(VmRuntimeError::OutOfBoundsError {
                address: addr as u64,
            });
        }

        Ok(*(self.0.get(addr).expect("range already checked")))
    }

    pub fn dump_hex(&self) -> String {
        lib_utils::dump_hex(&self.0)
    }
}
