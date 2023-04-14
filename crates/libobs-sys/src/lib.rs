#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const LIBOBS_API_VER: u32 =
    LIBOBS_API_MAJOR_VER << 24 | LIBOBS_API_MINOR_VER << 16 | LIBOBS_API_PATCH_VER;
