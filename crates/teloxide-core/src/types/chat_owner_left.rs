use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object describes a service message about the owner of a chat leaving
/// it.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerleft).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerLeft {
    /// The user which will be the new owner of the chat if the previous owner
    /// does not return to the chat.
    pub new_owner: Option<User>,
}
