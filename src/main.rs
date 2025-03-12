use lib::prelude::MachineOptions;
use lib::types::memory::ByteUnits;

fn main() {
    let mut machine = MachineOptions::builder()
        .memory(ByteUnits::GibiBytes(16))
        .build_with_defaults();

    let v: Vec<u8> = vec![0b11; ByteUnits::GibiBytes(16).num_bytes() as usize];
    machine.load_binary(v.as_slice()).unwrap();

    dbg!(machine);
}
