use serde::{Deserialize, Serialize};

use crate::types::FileMeta;

/// This object describes a video quality.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Metadata of the video quality file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Video width.
    pub width: u32,

    /// Video height.
    pub height: u32,

    /// Codec that was used to encode the video.
    pub codec: String,
}

#[cfg(test)]
mod tests {
    use super::VideoQuality;
    use crate::types::{FileId, FileMeta, FileUniqueId};

    #[test]
    fn deserialize_file_size_into_file_meta() {
        let json = r#"{
            "file_id":"id2",
            "file_unique_id":"uid2",
            "file_size":100,
            "width":1280,
            "height":720,
            "codec":"h265"
        }"#;

        let quality: VideoQuality = serde_json::from_str(json).unwrap();

        assert_eq!(quality.file.size, 100);
        assert_eq!(quality.codec, "h265");
    }

    #[test]
    fn serialize_file_size_only_once() {
        let quality = VideoQuality {
            file: FileMeta {
                id: FileId("id2".into()),
                unique_id: FileUniqueId("uid2".into()),
                size: 100,
            },
            width: 1280,
            height: 720,
            codec: "h265".into(),
        };

        let json = serde_json::to_string(&quality).unwrap();

        assert_eq!(json.matches("\"file_size\"").count(), 1);
    }
}
