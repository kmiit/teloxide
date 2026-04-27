use serde::{Deserialize, Serialize};

/// Style of a keyboard button.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Danger,
    Success,
    Primary,
}

