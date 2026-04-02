//! # wraith-dom
//!
//! A minimal `#![no_std]` HTML parser with CSS selectors and Cloudflare
//! challenge detection/solving.
//!
//! Designed for environments without the standard library: bare-metal systems,
//! WebAssembly, embedded devices, and anywhere you need lightweight HTML
//! processing without pulling in a full browser engine.
//!
//! ## Features
//!
//! - **HTML parsing** -- tokenizer and tree builder producing a flat node arena
//! - **CSS selectors** -- tag, id, class, attribute, descendant combinators
//! - **Form detection** -- extract forms, inputs, and login form heuristics
//! - **Text extraction** -- visible text, title, and link extraction
//! - **Cloudflare IUAM bypass** (behind `cloudflare` feature flag) -- detect and
//!   solve Cloudflare "Under Attack Mode" challenge pages via `js-lite`
//!
//! ## Memory
//!
//! All allocations go through `alloc`. No external dependencies beyond `log`.

#![no_std]

extern crate alloc;

#[cfg(feature = "cloudflare")]
pub mod cloudflare;
pub mod forms;
pub mod parser;
pub mod selector;
pub mod text;

#[cfg(feature = "cloudflare")]
pub use cloudflare::{
    handle_cloudflare_response, is_cloudflare_challenge, solve_challenge,
    ChallengeResult, CloudflareCookie,
};
pub use forms::{find_forms, find_login_form, Form, FormInput};
pub use parser::{parse, Document, Node, NodeData};
pub use selector::{select, Selector};
pub use text::{extract_links, extract_text, extract_title};
