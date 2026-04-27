use serde::{Deserialize, Serialize};

use crate::types::Audio;

/// This object represents a user's profile audios.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    /// Total number of profile audios the target user has.
    pub total_count: u32,

    /// The list of profile audios.
    pub audios: Vec<Audio>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "total_count": 1,
            "audios": [
                {
                    "file_id": "id",
                    "file_unique_id": "uid",
                    "duration": 60,
                    "mime_type": null
                }
            ]
        }"#;

        let audios: UserProfileAudios = serde_json::from_str(json).unwrap();
        assert_eq!(audios.total_count, 1);
        assert_eq!(audios.audios.len(), 1);
    }
}
