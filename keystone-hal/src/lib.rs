#![cfg_attr(not(test), no_std)]
// `asm!` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub mod edge;
pub mod edge_syscall;
#[cfg(feature = "rt")]
pub mod mem;
#[cfg(feature = "rt")]
pub mod rt;
pub mod vm;
#[cfg(feature = "rt")]
pub use rt::*;

pub use keystone_cfg as cfg;
