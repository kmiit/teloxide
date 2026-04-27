use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds, VideoQuality};

/// This object represents a video file.
///
/// [The official docs](https://core.telegram.org/bots/api#video).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Video {
    /// Metadata of the video file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Video width as defined by sender.
    pub width: u32,

    /// Video height as defined by sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: Seconds,

    /// Video thumbnail.
    pub thumbnail: Option<PhotoSize>,

    /// Available sizes of the cover of the video in the message
    pub cover: Option<Vec<PhotoSize>>,

    /// Timestamp in seconds from which the video will play in the message
    pub start_timestamp: Option<Seconds>,

    /// Available quality levels of the video
    pub qualities: Option<Vec<VideoQuality>>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// Mime type of a file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_with_qualities() {
        let json = r#"{
            "file_id":"id",
            "file_unique_id":"uid",
            "file_size":123,
            "width":1920,
            "height":1080,
            "duration":60,
            "mime_type": null,
            "qualities":[
                {
                    "file_id":"id2",
                    "file_unique_id":"uid2",
                    "file_size":100,
                    "width":1280,
                    "height":720,
                    "codec":"h265"
                }
            ]
        }"#;
        let video: Video = serde_json::from_str(json).unwrap();
        assert_eq!(video.qualities.as_ref().unwrap().len(), 1);
        assert_eq!(video.qualities.as_ref().unwrap()[0].codec, "h265");
        assert_eq!(video.qualities.as_ref().unwrap()[0].file.size, 100);
    }
}
