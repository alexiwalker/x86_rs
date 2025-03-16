impl UnsignedInteger for u8 {}

trait UnsignedInteger {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}


/*
Reference:

2.1.3 ModR/M and SIB Bytes
Many instructions that refer to an operand in memory have an addressing-form specifier byte (called the ModR/M
byte) following the primary opcode. The ModR/M byte contains three fields of information:
• The mod field combines with the r/m field to form 32 possible values: eight registers and 24 addressing modes.
• The reg/opcode field specifies either a register number or three more bits of opcode information. The purpose
of the reg/opcode field is specified in the primary opcode.
• The r/m field can specify a register as an operand or it can be combined with the mod field to encode an
addressing mode. Sometimes, certain combinations of the mod field and the r/m field are used to express
opcode information for some instructions.
Certain encodings of the ModR/M byte require a second addressing byte (the SIB byte). The base-plus-index and
scale-plus-index forms of 32-bit addressing require the SIB byte. The SIB byte includes the following fields:
• The scale field specifies the scale factor.
• The index field specifies the register number of the index register.
• The base field specifies the register number of the base register.
See Section 2.1.5 for the encodings of the ModR/M and SIB bytes.
*/
pub enum OperandType<T:UnsignedInteger> {
        Register(T),
        Memory(T)
}


pub enum X86Opcode {
    Add00(OperandType<u8>,OperandType<u8>),
    Add01(OperandType<u8>,OperandType<u8>),
    AddR8M8(OperandType<u8>,OperandType<u8>),
    Add03(u8,u8),
    AddAlImm8(u8,u8),
    AddEaxImm(u8,u8),
    Push(u8) /*u8 = register?*/


}