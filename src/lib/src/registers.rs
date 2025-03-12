use crate::register_aliases::Alias;
use lib_types::error::{SafetyResult, VmRuntimeError};
use std::fmt;
use std::ops::Deref;

pub type RegisterAliasingWidth = u16;

#[derive(Clone, Debug)]
// equivalent to 16 x 64 bit registers
// created as u8 here to make addressing specific segments and aliases easier
pub enum RegisterWidth {
    Fixed(RegisterAliasingWidth),
    Variable,
}

#[derive(Clone)]

pub struct Registers<const N: usize>(pub(crate) [u8; N], RegisterWidth);

impl<const N: usize> Registers<N> {
    /// checks that the alignment of the alias provided aligns with the size, offsets and alignment of a register
    /// shouldnt allow the register to be read by weird offsets eg in a 4 byte register, read the bytes [_, a, b, _]
    /// as a and b are not aligned. [a, b, _, _] and [_, _, a, b] would be though
    #[inline(always)]
    #[allow(unused)] /* allow unused because of feature flags */
    fn safety_check(&self, alias: &Alias) -> Result<(), VmRuntimeError> {
        let width = alias.width;
        let offset = alias.offset;

        let start = width * offset;
        let end = start + width;

        if alias.width < 8 {
            return Err(VmRuntimeError::InvalidAlias {
                offset,
                width,
            });
        }

        if (end / 8) as usize > self.0.len() {
            return Err(VmRuntimeError::RegisterAliasOverrun {
                offset,
                width,
                alignment: 8,
            });
        }

        if width % 8 != 0 {
            return Err(VmRuntimeError::InvalidAlias { offset, width });
        }

        let cannonical_width = &self.1;

        match cannonical_width {
            RegisterWidth::Fixed(w) => {
                if *w < alias.width {
                    return Err(VmRuntimeError::RegisterAliasOverrun {
                        offset: alias.offset,
                        width: alias.width,
                        alignment: *w,
                    });
                }

                if start % w != 0 {
                    return Err(VmRuntimeError::InvalidAlias { offset, width });
                }
            }
            RegisterWidth::Variable => {}
        }

        Ok(())
    }

    pub fn canonical_width(&self) -> RegisterAliasingWidth {
        match self.1 {
            RegisterWidth::Fixed(v) => v,
            RegisterWidth::Variable => 8,
        }
    }

    pub fn write_u8(&mut self, alias: Alias, val: u8) -> SafetyResult<()> {
        let offset = alias.offset;

        #[cfg(feature = "safety_checks")]
        {
            let width = alias.width;
            if width % 8 != 0 {
                return Err(VmRuntimeError::InvalidAlias { offset, width });
            }
            
            if alias.offset as usize > self.0.len() {
                return Err(VmRuntimeError::RegisterAliasOverrun {  offset, width, alignment: alias.offset });
            }
        }

        let i = offset as usize;
        let mem = &mut self.0;
        mem[i] = val;


        #[cfg(feature = "safety_checks")]
        {
            Ok(())
        }
        #[cfg(not(feature = "safety_checks"))]
        {
            ()
        }
    }

    pub fn write_u16(
        &mut self,
        alias: Alias,
        val: RegisterAliasingWidth,
    ) -> SafetyResult<()> {
        #[cfg(feature = "safety_checks")]
        {
            self.safety_check(&alias)?
        }

        let mem: &mut [u8] = self.0.as_mut_slice();
        mem[alias.range()].copy_from_slice(&val.to_le_bytes());

        #[cfg(feature = "safety_checks")]
        {
            Ok(())
        }
        #[cfg(not(feature = "safety_checks"))]
        {
            ()
        }
    }

    pub fn write_u32(&mut self, alias: Alias, val: u32) -> SafetyResult<()> {
        #[cfg(feature = "safety_checks")]
        {
            self.safety_check(&alias)?
        }

        let mem: &mut [u8] = self.0.as_mut_slice();
        mem[alias.range()].copy_from_slice(&val.to_le_bytes());

        #[cfg(feature = "safety_checks")]
        {
            Ok(())
        }
        #[cfg(not(feature = "safety_checks"))]
        {
            ()
        }
    }

    pub fn write_u64(&mut self, alias: Alias, val: u64) -> SafetyResult<()>{
        #[cfg(feature = "safety_checks")]
        {
            self.safety_check(&alias)?
        }

        let mem: &mut [u8] = self.0.as_mut_slice();
        mem[alias.range()].copy_from_slice(&val.to_le_bytes());

        #[cfg(feature = "safety_checks")]
        {
            Ok(())
        }
        #[cfg(not(feature = "safety_checks"))]
        {
            ()
        }
    }

    pub fn new(width: RegisterWidth) -> Self {
        Registers([0; N], width)
    }

    pub fn read_u8(&self, alias: Alias) -> SafetyResult<u8> {
        let offset = alias.offset;
        let memory = self.0.as_slice();

        #[cfg(feature = "safety_checks")]
        {
            let width = alias.width;
            if width % 8 != 0 {
                return Err(VmRuntimeError::InvalidAlias { offset, width });
            }

            let cannonical_width = &self.1;

            match cannonical_width {
                RegisterWidth::Fixed(w) => {
                    if *w < 8 {
                        return Err(VmRuntimeError::RegisterAliasOverrun {
                            offset: alias.offset,
                            width: alias.width,
                            alignment: *w,
                        });
                    }
                }
                RegisterWidth::Variable => {
                    // nothing to check here
                }
            }

            let v = memory.get(offset as usize);

            match v {
                None => {
                    return {
                        match self.1 {
                            RegisterWidth::Fixed(w) => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: w,
                            }),
                            RegisterWidth::Variable => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: 8,
                            }),
                        }
                    };
                }
                Some(v) => Ok(*v),
            }
        }

        #[cfg(not(feature = "safety_checks"))]
        memory[offset as usize]
    }

    pub fn read_u16(&self, alias: Alias) -> SafetyResult<u16> {
        let memory = self.0.as_slice();
        let offset = alias.offset;
        #[cfg(feature = "safety_checks")]
        {
            self.safety_check(&alias)?;

            let v = &memory.get(alias.range());

            match v {
                None => {
                    return {
                        match self.1 {
                            RegisterWidth::Fixed(w) => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: w,
                            }),
                            RegisterWidth::Variable => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: 16,
                            }),
                        }
                    };
                }
                Some(v) => {
                    let derefed = *v;

                    if derefed.len() != 2 {
                        return Err(VmRuntimeError::RegisterAliasOverrun {
                            offset: alias.offset,
                            width: alias.width,
                            alignment: 16,
                        });
                    }

                    let num = u16::from_le_bytes([derefed[0], derefed[1]]);
                    Ok(num)
                }
            }
        }

        #[cfg(not(feature = "safety_checks"))]
        {
            let bytes: &[u8] = &memory[alias.range()];
            u16::from_le_bytes(bytes.try_into().unwrap())
        }
    }

    pub fn read_u32(&self, alias: Alias) -> SafetyResult<u32> {
        let memory = self.0.as_slice();
        let offset = alias.offset;
        #[cfg(feature = "safety_checks")]
        {
            self.safety_check(&alias)?;

            let v = &memory.get(alias.range());

            match v {
                None => {
                    return {
                        match self.1 {
                            RegisterWidth::Fixed(w) => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: w,
                            }),
                            RegisterWidth::Variable => Err(VmRuntimeError::RegisterAliasOverrun {
                                offset: alias.offset,
                                width: alias.width,
                                alignment: 32,
                            }),
                        }
                    };
                }
                Some(v) => {
                    let derefed = *v;

                    if derefed.len() != 4 {
                        return Err(VmRuntimeError::RegisterAliasOverrun {
                            offset: alias.offset,
                            width: alias.width,
                            alignment: 8,
                        });
                    }

                    let num = u32::from_le_bytes([derefed[0], derefed[1], derefed[2], derefed[3]]);
                    Ok(num)
                }
            }
        }

        #[cfg(not(feature = "safety_checks"))]
        {
            let bytes: &[u8] = &memory[alias.range()];
            u32::from_le_bytes(bytes.try_into().unwrap())
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
