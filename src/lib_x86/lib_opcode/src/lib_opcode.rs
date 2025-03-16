pub mod opcodes;
pub mod encode;
pub mod decode;

mod prelude {
    pub use crate::decode::*;
    pub use crate::encode::*;
    pub use crate::opcodes::*;
}