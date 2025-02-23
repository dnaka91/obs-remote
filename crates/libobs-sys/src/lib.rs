#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::redundant_static_lifetimes,
    clippy::suspicious_doc_comments,
    clippy::upper_case_acronyms,
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const LIBOBS_API_VER: u32 =
    (LIBOBS_API_MAJOR_VER << 24) | (LIBOBS_API_MINOR_VER << 16) | LIBOBS_API_PATCH_VER;
