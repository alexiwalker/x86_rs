use std::fmt;
use std::ops::Deref;
use lib_types::memory::ByteUnits;

#[derive(Clone)]
// equivalent to 16 x 64 bit registers
// created as u8 here to make addressing specific segments and aliases easier
pub(crate) struct Registers<const N: usize>(pub(crate) [u8; N]);

impl<const N: usize> Default for Registers<N> {
    fn default() -> Self {
        Registers([0; N])
    }
}

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

#[derive(Debug, Clone)]
pub(crate) struct Fpu {
    pub(crate) registers:  Registers<80> /* 8 x 80 bit registers, represented by u8s*/
}

impl Default for Fpu {
    fn default() -> Self {
        Self {
            registers: Default::default(),
        }
    }
}

impl ContiguousMemory {
    pub fn with_size(size: &ByteUnits) -> Self {
        Self(vec![0; size.num_bytes() as usize])
    }
}