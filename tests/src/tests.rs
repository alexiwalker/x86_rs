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
