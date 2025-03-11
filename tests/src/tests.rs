#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       // check: crate compiles, no logic in test
        assert!(true)
    }
}



#[cfg(test)]
mod flags {
    use lib::flags::*;

    #[test]
    fn set_flags(){
        let mut flags = 0u64;
        let f = RFlags::Carry | RFlags::Interrupt;

        flags |= f;

        assert!(RFlags::is_set(flags, RFlags::Carry));
        assert!(RFlags::is_set(flags, RFlags::Interrupt));

    }
    #[test]
    fn clear_flags(){
        let mut flags = 0u64;
        let f = RFlags::Carry | RFlags::Interrupt;

        flags |= f;

        assert!(RFlags::is_set(flags, RFlags::Carry));
        assert!(RFlags::is_set(flags, RFlags::Interrupt));

        flags.clear(RFlags::Carry);

        assert!(!RFlags::is_set(flags, RFlags::Carry));
        assert!(RFlags::is_set(flags, RFlags::Interrupt));

    }
}

#[cfg(test)]
mod machine {
    use lib::prelude::*;

    #[test]
    fn machine_builder_works() {
        let machine = MachineOptions::builder()
            .memory(ByteUnits::Bytes(512))
            .build_machine();


        dbg!(machine);

        // dbg!(machine);
    }
}

#[cfg(test)]
mod intrinsics {

    fn test_intrinsic(machine: &mut X86Machine) -> () {
        println!("test_intrinsic. machine has bytes: {}", machine.assigned_memory.num_bytes());
    }

    use lib::functions::Intrinsic;
    use lib::prelude::{ByteUnits, X86Machine};

    #[test]
    fn can_convert_function() {

        let mut machine = X86Machine::builder()
            .memory(ByteUnits::KibiBytes(512))
            .build_with_defaults();

        let intrinsic = Intrinsic::from_ptr(test_intrinsic);

        test_intrinsic(&mut machine);

    }

}



#[cfg(test)]
mod memory {
    use std::fs;
    use lib::memory::ContiguousMemory;
    use lib_types::memory::ByteUnits;

    const DEFAULT_BYTE_SIZE:ByteUnits = ByteUnits::MebiBytes(1);
    #[test]
    fn alloc_contiguous_memory() {
        let mem = ContiguousMemory::with_size(&DEFAULT_BYTE_SIZE);

        let default_bytes = DEFAULT_BYTE_SIZE.num_bytes();

        let mem_size = mem.len() as u64;

        assert_eq!(mem_size,default_bytes)

    }


    #[test]
    fn zero_initialised() {
        let mem = ContiguousMemory::with_size(&DEFAULT_BYTE_SIZE);
        let part_1 = rand::random_range(0..512) as usize;
        let part_2 = rand::random_range(1024..mem.len()-1);

        for i in part_1 .. part_2 {
            match mem.get(i) {
                None => {
                    assert!(false);
                }
                Some(v) => {
                    assert_eq!(*v,0);
                }
            }
        }
    }


    #[test]
    fn write_range() {
       for _ in 0..10000 {
            let mut mem = ContiguousMemory::with_size(&DEFAULT_BYTE_SIZE);

            let part_1 = rand::random_range(10..512) as usize; //gives space on the low end to read the low byte later
            let part_2 = rand::random_range(1024..2048);

            let range_size = part_2-part_1;

            let test_byte = 0xffu8;

            let data_block = vec![test_byte; range_size];

            let res = mem.write(part_1, &data_block);
            assert!(res.is_ok());

            let byte_below = part_1-1;

            let low_byte = mem.read_byte(byte_below);

            assert!(low_byte.is_ok());
            assert_eq!(low_byte.unwrap(), 0);


            let start_byte = mem.read_byte(part_1);

            assert!(start_byte.is_ok());
            assert_eq!(start_byte.unwrap(), test_byte);

            let slight_offset = part_1+8;

            let offset_byte = mem.read_byte(slight_offset);

            assert!(offset_byte.is_ok());
            assert_eq!(offset_byte.unwrap(), test_byte);

            let end_byte = mem.read_byte(part_2 - 1 ); /* -1 to account for zero indexing*/

            assert!(end_byte.is_ok());
            assert_eq!(end_byte.unwrap(), test_byte);

            let after_end = mem.read_byte(part_2);

            assert!(after_end.is_ok());
            assert_eq!(after_end.unwrap(), 0);

            let end_offset = part_2+8;

            let end_byte = mem.read_byte(end_offset);
            assert!(end_byte.is_ok());
            assert_eq!(end_byte.unwrap(), 0);

        }
    }


    #[test]
    fn write_fixed_range() {
        let mut mem = ContiguousMemory::with_size(&DEFAULT_BYTE_SIZE);

        let start_point = 8usize;
        let end_point = 16usize;

        let range_size = 8usize;


        let test_byte = 0xffu8;

        let data_block = vec![test_byte; range_size];

        let res = mem.write(start_point, &data_block);

        assert!(res.is_ok());

        let byte_below = start_point -1;

        let low_byte = mem.read_byte(byte_below);

        assert!(low_byte.is_ok());
        assert_eq!(low_byte.unwrap(), 0);


        let start_byte = mem.read_byte(start_point);

        assert!(start_byte.is_ok());
        assert_eq!(start_byte.unwrap(), test_byte);

        let slight_offset = start_point +3;

        let offset_byte = mem.read_byte(slight_offset);

        assert!(offset_byte.is_ok());
        assert_eq!(offset_byte.unwrap(), test_byte);

        let end_byte = mem.read_byte(end_point - 1 ); /* -1 to account for zero indexing*/

        assert!(end_byte.is_ok());
        assert_eq!(end_byte.unwrap(), test_byte);

        let after_end = mem.read_byte(end_point);

        assert!(after_end.is_ok());
        assert_eq!(after_end.unwrap(), 0);

        let end_offset = end_point +8;

        let end_byte = mem.read_byte(end_offset);
        assert!(end_byte.is_ok());
        assert_eq!(end_byte.unwrap(), 0);

    }



}


#[cfg(test)]
mod registers {
    use lib::register_aliases::Alias;
    use lib::registers::*;

    const ALIAS_8_BIT:Alias = Alias {
        width:8,
        offset:3, //4th byte: 0 indexed
    };

    const ALIAS_16_BIT_1:Alias = Alias {
        width:16,
        offset:1, //offset of 1x16 bit register, meaning we start at the 17th bit
    };

    const ALIAS_16_BIT_2:Alias = Alias {
        width:16,
        offset:7, //offset of 1x16 bit register, meaning we start at the 17th bit
    };
    #[test]
    fn test_8x8_registers(){
        /* 8 x 8 bit registers*/
        let mut register = Registers::<8>::new(RegisterWidth::Fixed(8));

        register.write_u8(ALIAS_8_BIT, 255).unwrap();

        let s = register.dump_hex();

        println!("{s}");

        let v = register.read_u8(ALIAS_8_BIT);

        assert!(v.is_ok());

        assert_eq!(v.unwrap(), 255);

    }

    #[test]
    fn test_8x16_registers(){
        /* 8 x 16 bit registers*/
        let mut register = Registers::<{( 8 * 16 ) / 8 }>::new(RegisterWidth::Fixed(16));
        let s = register.dump_hex();

        println!("{s}");

        register.write_u16( ALIAS_16_BIT_1, 65535 ).unwrap();


        register.write_u16( ALIAS_16_BIT_2, 12121 ).unwrap();


        let s_after = register.dump_hex();

        println!("{s_after}");

    }
}