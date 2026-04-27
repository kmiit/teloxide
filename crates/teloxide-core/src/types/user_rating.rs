use serde::{Deserialize, Serialize};

/// This object describes the rating of a user in a private chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// The current rating level of the user.
    pub level: u32,

    /// The current rating of the user.
    pub rating: u32,

    /// The rating required for the current level.
    pub current_level_rating: u32,

    /// The rating required for the next level.
    pub next_level_rating: Option<u32>,
}
