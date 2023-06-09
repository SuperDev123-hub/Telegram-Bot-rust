use crate::entities::user::User;
use serde::{Deserialize, Serialize};

///This object represents an answer of a user in a non-anonymous poll.
///API Reference: [link](https://core.telegram.org/bots/api/#pollanswer)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    ///Unique poll identifier
    pub poll_id: String,

    ///The user, who changed the answer to the poll
    pub user: User,

    ///0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<i64>,
}
// Divider: all content below this line will be preserved after code regen
