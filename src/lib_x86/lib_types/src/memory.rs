/// Enum used to pass values referencing amounts of memory so that there is no confusion about units
///
#[derive(Debug, Clone)]
pub enum ByteUnits {
    /// b (1 x bytes)
    Bytes(u64),

    /// KB (1000 x Bytes
    KiloBytes(u64),

    /// KiB (1024 x Bytes)
    KibiBytes(u64),

    /// MB ( 1000 x Kilobyte)
    MegaBytes(u64),

    /// MiB (1024 x KibiBytes)
    MebiBytes(u64),

    /// GB ( 1000 x MegaBytes)
    GigaBytes(u64),

    /// GiB (1024 x MebiBytes)
    GibiBytes(u64),
}

impl ByteUnits {
    /// Only exposed cast is to_bytes, as other sizes would require floats to represent in some cases
    /// and all actual usages will be converted to bytes eg requesting certain sizes of memory
    pub fn num_bytes(&self) -> u64 {
        match self {
            ByteUnits::Bytes(v) => *v,
            ByteUnits::KiloBytes(v) => *v * 1000,
            ByteUnits::KibiBytes(v) => *v * 1024,
            ByteUnits::MegaBytes(v) => *v * (1000 * 1000),
            ByteUnits::MebiBytes(v) => *v * (1024 * 1024),
            ByteUnits::GigaBytes(v) => *v * (1000 * 1000 * 1000),
            ByteUnits::GibiBytes(v) => *v * (1024 * 1024 * 1024),
        }
    }

    pub fn to_bytes(self) -> Self {
        match self {
            ByteUnits::Bytes(_) => self,
            ByteUnits::KiloBytes(v) => Self::Bytes(v * 1000),
            ByteUnits::KibiBytes(v) => Self::Bytes(v * 1024),
            ByteUnits::MegaBytes(v) => Self::Bytes(v * (1000 * 1000)),
            ByteUnits::MebiBytes(v) => Self::Bytes(v * (1024 * 1024)),
            ByteUnits::GigaBytes(v) => Self::Bytes(v * (1000 * 1000 * 1000)),
            ByteUnits::GibiBytes(v) => Self::Bytes(v * (1024 * 1024 * 1024)),
        }
    }
}
