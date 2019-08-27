//! Target triple support.

#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else,
        clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self
    )
)]
#![no_std]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
extern crate alloc as std;
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate failure_derive;

mod host;
mod parse_error;
mod targets;
#[macro_use]
mod triple;

pub use self::host::HOST;
pub use self::parse_error::ParseError;
pub use self::targets::{Architecture, BinaryFormat, Environment, OperatingSystem, Vendor};
pub use self::triple::{CallingConvention, Endianness, PointerWidth, Triple};
