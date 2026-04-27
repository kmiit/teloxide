use serde::{Deserialize, Serialize};

use crate::types::RequestId;

/// This object defines the parameters for the creation of a managed bot.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the
    /// message.
    pub request_id: RequestId,

    /// Suggested name for the bot.
    pub suggested_name: Option<String>,

    /// Suggested username for the bot.
    pub suggested_username: Option<String>,
}
