#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // TODO: investigate
#![allow(clippy::upper_case_acronyms)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const LIBOBS_API_VER: u32 =
    LIBOBS_API_MAJOR_VER << 24 | LIBOBS_API_MINOR_VER << 16 | LIBOBS_API_PATCH_VER;
