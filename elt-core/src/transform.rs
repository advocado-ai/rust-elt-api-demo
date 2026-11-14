//! Raw CSV-shaped rows -> normalised `MatchRecord`.
//!
//! - `parse`: the field parsers (task T1)
//! - `assemble`: `transform()`, which uses the parsers (task T2)
//! - `tests`: protected unit tests

// TEMP (until task T2 lands): the parsers are only called from tests while transform() is a stub.
#[allow(dead_code)]
mod parse;

mod assemble;
pub use assemble::transform;

#[cfg(test)]
mod tests;
