#![allow(unused,dead_code)]
//todo remove global allow after initial development

mod tests;
pub mod x86;
pub mod flags;
pub mod functions;
pub mod register_aliases;
pub mod memory;
pub mod hardware;
pub mod registers;

pub mod prelude {
    pub use crate::x86::*;
    pub use crate::flags::*;
    pub use crate::types::*;
    pub use crate::functions::*;
    pub use crate::memory::*;
}

pub mod types {
    pub use lib_types::*;
    pub use crate::types::memory::*;
}