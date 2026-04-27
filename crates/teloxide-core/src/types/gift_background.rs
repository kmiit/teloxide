use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the colors of the background of a unique gift.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// The color in the center of the background in RGB format.
    pub center_color: Rgb,

    /// The color on the edges of the background in RGB format.
    pub edge_color: Rgb,

    /// The color to be applied to the text in RGB format.
    pub text_color: Rgb,
}
