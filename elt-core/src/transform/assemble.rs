use crate::error::TransformError;
use crate::types::{MatchRecord, RawMatch};

/// Converts one raw CSV-shaped row into a normalised `MatchRecord`.
///
/// STUB (T2): not implemented yet; the agent task `transform` fills this in.
/// The first hand-written draft is in git history (commit 103f900). Intended
/// behaviour, taken from that draft's notes:
/// - `id` is `{tourney_id}_{match_num}`
/// - every `MatchRecord` field is parsed from its `RawMatch` field with the
///   helpers below, and any parse failure is returned as a `TransformError`
pub fn transform(_raw: RawMatch) -> Result<MatchRecord, TransformError> {
    Err(TransformError::MissingField(
        "transform is not implemented yet (task T2)".to_string(),
    ))
}
