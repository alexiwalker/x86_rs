mod tests;
pub mod x86;
pub mod flags;

pub mod prelude {
    pub use crate::x86::*;
    pub use crate::flags::*;
    pub use crate::types::*;
}

pub mod types {
    pub use lib_types::*;
    pub use crate::types::memory::*;
}