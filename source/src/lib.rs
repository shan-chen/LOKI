//! LOKI fuzz core

// #![deny(
//     unused,
//     unused_imports,
//     unused_features,
//     unused_extern_crates,
//     unused_import_braces,
//     unstable_features,
//     unused_crate_dependencies,
//     unused_lifetimes,
//     unused_qualifications,
//     bare_trait_objects,
//     future_incompatible,
//     absolute_paths_not_starting_with_crate,
//     keyword_idents,
//     meta_variable_misuse,
//     non_ascii_idents,
//     unreachable_pub,
//     nonstandard_style,
//     dead_code,
//     deprecated,
//     rustdoc::broken_intra_doc_links,
//     trivial_casts,
//     trivial_numeric_casts
// )]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]

#[macro_use]
#[allow(missing_docs)]
pub mod verbose;
pub mod decode_map_to_buffer;
pub mod encode_map_to_buffer;
pub mod encoding;
pub mod engine;
pub mod error;
pub mod global_definition;
pub mod hash_functions;
pub mod loki_message;
pub mod loki_type;
pub mod message_pool;
pub mod mutator;
pub mod neighbour;
pub mod protos;
pub mod signature_functions;
pub mod state_model;
pub mod target_strategy;
pub mod user_interface;
pub mod utils;
