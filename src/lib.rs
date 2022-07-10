//! This is a type-safe HTML generator.
//!
//! # Example
//!
//! ```rust
//! # use htmlgen::html;
//! let doc = html(&|h| {
//!     h.head(&|head| {
//!         head.title("Hello world!");
//!     });
//! });
//! assert_eq!(doc, "<!DOCTYPE html><html><head><title>Hello world!</title></head></html>")
//! ```
//!
pub use body::{body, Body};
pub use head::{head, Head};
pub use html::{html, Html};

mod html;
mod body;
mod head;
