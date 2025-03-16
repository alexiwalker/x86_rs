#![allow(unused, dead_code)]
//todo remove global allow after initial development

pub mod flags;
pub mod functions;
pub mod hardware;
pub mod memory;
pub mod register_aliases;
pub mod registers;
pub mod x86;
pub mod builders;

pub mod prelude {
    pub use crate::flags::*;
    pub use crate::functions::*;
    pub use crate::memory::*;
    pub use crate::types::*;
    pub use crate::x86::*;
}

pub mod types {
    pub use crate::types::memory::*;
    pub use lib_types::*;
}
