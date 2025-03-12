use crate::error::VmRuntimeError;

pub trait DescriptorLike {
    fn write(&mut self, buf: &[u8]) -> Result<u64, VmRuntimeError>;
    fn read(&mut self, buf: &mut [u8]) -> Result<u64, VmRuntimeError>;
}
