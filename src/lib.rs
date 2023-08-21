//! Spacepacket EDS Library
//!
//! Implements an EDS model for CCSDS 876.0-B-1 ([Blue Book](https://public.ccsds.org/Pubs/876x0b1.pdf))

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![warn(unused_imports)]
#![allow(missing_copy_implementations)]
#![deny(missing_docs)]

pub mod codegen;
pub mod eds;
pub mod expr;
pub mod parse;
