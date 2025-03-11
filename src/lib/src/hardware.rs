use crate::memory::ContiguousMemory;

pub struct VirtualHardware {

    pub device_id: usize, // todo should this be an int id or a string? will need to look at this

    pub memory: ContiguousMemory


}