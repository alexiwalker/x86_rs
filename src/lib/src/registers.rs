use std::ops::Deref;
use std::fmt;
use std::io::Read;
use lib_types::error::VmRuntimeError;
use crate::register_aliases::Alias;

#[derive(Clone, Debug)]
// equivalent to 16 x 64 bit registers
// created as u8 here to make addressing specific segments and aliases easier
pub enum RegisterWidth {
    Fixed(u8),
    Variable
}

#[derive(Clone)]

pub struct Registers<const N: usize>(pub(crate) [u8; N], RegisterWidth);

impl<const N: usize>  Registers<N> {
    pub fn canonical_width(&self)->u8 {
        match self.1 {
            RegisterWidth::Fixed(v) => {
                v
            }
            RegisterWidth::Variable => {
                8
            }
        }
    }

    pub fn write_u8(&mut self, alias:Alias, val:u8) ->Result<(),VmRuntimeError>{
        let offset = alias.offset;


        #[cfg(feature = "safety_checks")]{
            let width = alias.width;
            if offset % 8 != 0 {
                return Err(VmRuntimeError::InvalidAlias{offset, width})
            }
        }

        let i = offset as usize;
        let mut mem = &mut self.0;
        mem[i] = val;

        Ok(())

    }

    pub fn write_u16(&mut self, alias:Alias, val:u16) ->Result<(),VmRuntimeError>{
        let offset = alias.offset;


        #[cfg(feature = "safety_checks")]{
            let width = alias.width;
            if offset % 8 != 0 {
                return Err(VmRuntimeError::InvalidAlias{offset, width})
            }

            let cannonical_width = &self.1;

            match cannonical_width {
                RegisterWidth::Fixed(w) => {
                    if *w < 16 {
                        return Err(VmRuntimeError::RegisterAlasOverrun {offset: alias.offset, width: alias.width, alignment: *w })
                    }
                }
                RegisterWidth::Variable => {
                    // nothing to check here
                }
            }
        }

        let mut mem: &mut [u8] = self.0.as_mut_slice();
        let i = ((offset*alias.width)/8) as usize;
        mem[i..i+2].copy_from_slice(&val.to_le_bytes());

        Ok(())

    }

    pub fn new(width:RegisterWidth) -> Self {
        Registers([0; N], width)
    }

    pub fn read_u8(&self, alias:Alias) -> Result<u8,VmRuntimeError> {
        let offset = alias.offset;

        #[cfg(feature = "safety_checks")]
        {
            let width = alias.width;
            if offset % 8 != 0 {
                return Err(VmRuntimeError::InvalidAlias{offset, width})
            }

            let cannonical_width = &self.1;

            match cannonical_width {
                RegisterWidth::Fixed(w) => {
                    if *w < 16 {
                        return Err(VmRuntimeError::RegisterAlasOverrun {offset: alias.offset, width: alias.width, alignment: *w })
                    }
                }
                RegisterWidth::Variable => {
                    // nothing to check here
                }
            }
        };

        let memory = self.0.as_slice();

        #[cfg(feature = "safety_checks")]
        {
            let v = memory.get(offset as usize);

            match v {
                None => {
                    return {
                        match self.1 {
                            RegisterWidth::Fixed(w) => {
                                Err(VmRuntimeError::RegisterAlasOverrun {offset: alias.offset, width: alias.width, alignment: *w })
                            }
                            RegisterWidth::Variable => {
                                Err(VmRuntimeError::RegisterAlasOverrun {offset: alias.offset, width: alias.width, alignment: 8 })
                            }
                        }
                    };
                }
                Some(v) => {
                    Ok(*v)
                }
            }
        }

        #[cfg(not(feature = "safety_checks"))]
        {
            Ok(memory[offset as usize])
        }

    }


    pub fn dump_hex(&self) -> String {
        lib_utils::dump_hex_unpadded(&self.0)
    }


}
impl<const N: usize> Default for Registers<N> {
    fn default() -> Self {
        Registers([0; N], RegisterWidth::Fixed(16))
    }
}

impl<const N: usize> Deref for Registers<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for Registers<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::format_truncated_hex(f, &self.0)
    }
}

impl<const N: usize> fmt::Display for Registers<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lib_utils::format_truncated_hex(f, &self.0)
    }
}