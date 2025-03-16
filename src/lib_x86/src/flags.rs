use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum RFlags {
    /*

    FLAGS register bits

    */
    Carry = 1 << 0,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_1 = 1 << 1,

    Parity = 1 << 2,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_2 = 1 << 3,

    AuxCarry = 1 << 4,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_3 = 1 << 5,

    Zero = 1 << 6,
    Sign = 1 << 7,
    Trap = 1 << 8,
    Interrupt = 1 << 9,
    Direction = 1 << 10,
    Overflow = 1 << 11,
    IOPrivilegeLevelLow = 1 << 12,
    IOPrivilegeLevelHigh = 1 << 13,
    NestedTask = 1 << 14,
    Mode = 1 << 15,

    /*

    EFLAGS register bits

    */
    Resume = 1 << 16,
    Virtual8086 = 1 << 17,
    AlignmentCheck = 1 << 18,
    VirtualInterrupt = 1 << 19,
    VirtualInterruptPending = 1 << 20,

    CanUseCpuidInstruction = 1 << 21,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_4 = 1 << 22,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_5 = 1 << 23,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_6 = 1 << 24,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_7 = 1 << 25,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_8 = 1 << 26,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_9 = 1 << 27,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_10 = 1 << 28,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_11 = 1 << 29,

    AesKeyScheduleLoaded = 1 << 30,
    AlternateInstructionSetEnabled = 1 << 31,

    /*

    RFLAGS upper 32 bits (reserved)

    */
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_12 = 1 << 32,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_13 = 1 << 33,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_14 = 1 << 34,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_15 = 1 << 35,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_16 = 1 << 36,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_17 = 1 << 37,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_18 = 1 << 38,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_19 = 1 << 39,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_20 = 1 << 40,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_21 = 1 << 41,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_22 = 1 << 42,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_23 = 1 << 43,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_24 = 1 << 44,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_25 = 1 << 45,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_26 = 1 << 46,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_27 = 1 << 47,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_28 = 1 << 48,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_29 = 1 << 49,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_30 = 1 << 50,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_31 = 1 << 51,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_32 = 1 << 52,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_33 = 1 << 53,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_34 = 1 << 54,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_35 = 1 << 55,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_36 = 1 << 56,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_37 = 1 << 57,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_38 = 1 << 58,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_39 = 1 << 59,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_40 = 1 << 60,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_41 = 1 << 61,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_42 = 1 << 62,

    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    Reserved_43 = 1 << 63,
}

pub trait AsRFlags {
    fn set(&mut self, flag: RFlags);

    fn clear(&mut self, flag: RFlags);

    fn is_set(&self, flag: RFlags) -> bool;
}

/// Extension trait: allow RFlag-related operations to be called directly on u64 without newtyping it
impl AsRFlags for u64 {
    fn set(&mut self, flag: RFlags) {
        *self |= flag as u64
    }

    fn clear(&mut self, flag: RFlags) {
        *self &= !(flag as u64)
    }

    fn is_set(&self, flag: RFlags) -> bool {
        (flag as u64) & self != 0
    }
}

impl RFlags {
    /// Set a flag in the given flags register.
    pub fn set(flags: &mut u64, flag: RFlags) {
        *flags |= flag as u64;
    }

    /// Clear a flag in the given flags register.
    pub fn clear(flags: &mut u64, flag: RFlags) {
        *flags &= !(flag as u64);
    }

    /// Check if a flag is set in the given flags register.
    pub fn is_set(flags: u64, flag: RFlags) -> bool {
        flags & (flag as u64) != 0
    }
}

impl BitOrAssign<RFlags> for u64 {
    fn bitor_assign(&mut self, rhs: RFlags) {
        *self |= rhs as u64;
    }
}

impl BitOr<RFlags> for RFlags {
    type Output = u64;

    fn bitor(self, rhs: RFlags) -> Self::Output {
        rhs as u64 | self as u64
    }
}
