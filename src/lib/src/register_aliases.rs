use crate::flags::RFlags::Parity;

pub enum RegisterAlias {
    RAX,
    RCX,
    RBX,
    RDX,
    RSP,
    RBP,
    RSI,
    RDI,
    EAX,
    ECX,
    EBX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,
    AX,
    CX,
    BX,
    DX,
    SP,
    BP,
    SI,
    DI,
    AL,
    CL,
    BL,
    DL,
    SPL,
    BPL,
    SIL,
    DIL,

    R0D,
    R1D,
    R3D,
    R4D,
    R5D,
    R6D,
    R7D,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
}

pub(crate) struct Alias {
    offset: u8,
    width: u8,
}

impl RegisterAlias {

    pub fn describe(&self) -> &'static Alias {
         const a: Alias = Alias {
             offset:0,
             width: 0,
         };

        &a

        // match self {
        //     RegisterAlias::RAX => {}
        //     RegisterAlias::RCX => {}
        //     RegisterAlias::RBX => {}
        //     RegisterAlias::RDX => {}
        //     RegisterAlias::RSP => {}
        //     RegisterAlias::RBP => {}
        //     RegisterAlias::RSI => {}
        //     RegisterAlias::RDI => {}
        //     RegisterAlias::EAX => {}
        //     RegisterAlias::ECX => {}
        //     RegisterAlias::EBX => {}
        //     RegisterAlias::EDX => {}
        //     RegisterAlias::ESP => {}
        //     RegisterAlias::EBP => {}
        //     RegisterAlias::ESI => {}
        //     RegisterAlias::EDI => {}
        //     RegisterAlias::AX => {}
        //     RegisterAlias::CX => {}
        //     RegisterAlias::BX => {}
        //     RegisterAlias::DX => {}
        //     RegisterAlias::SP => {}
        //     RegisterAlias::BP => {}
        //     RegisterAlias::SI => {}
        //     RegisterAlias::DI => {}
        //     RegisterAlias::AL => {}
        //     RegisterAlias::CL => {}
        //     RegisterAlias::BL => {}
        //     RegisterAlias::DL => {}
        //     RegisterAlias::SPL => {}
        //     RegisterAlias::BPL => {}
        //     RegisterAlias::SIL => {}
        //     RegisterAlias::DIL => {}
        // }
    }

}