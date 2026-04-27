use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// This object describes the colors of a unique gift.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// The custom emoji identifier of the model.
    pub model_custom_emoji_id: CustomEmojiId,

    /// The custom emoji identifier of the symbol.
    pub symbol_custom_emoji_id: CustomEmojiId,

    /// The color in the center of the gift's gradient for the light theme in
    /// RGB format.
    pub light_theme_main_color: Rgb,

    /// The other colors of the gift's gradient for the light theme in RGB
    /// format.
    pub light_theme_other_colors: Vec<Rgb>,

    /// The color in the center of the gift's gradient for the dark theme in RGB
    /// format.
    pub dark_theme_main_color: Rgb,

    /// The other colors of the gift's gradient for the dark theme in RGB
    /// format.
    pub dark_theme_other_colors: Vec<Rgb>,
}
