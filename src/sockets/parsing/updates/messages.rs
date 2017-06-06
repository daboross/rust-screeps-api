//! Update parsing for user messages and conversation updates.
use std::marker::PhantomData;

/// Specification on whether a message is incoming or outgoing.
#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MessageDirectionType {
    /// Incoming messages: messages sent by someone other than the subscribed user.
    #[serde(rename = "in")]
    Incoming,
    /// Outgoing messages: messages sent by the subscribed user.
    #[serde(rename = "out")]
    Outgoing,
}

/// Content of a newly sent or received message update.
#[derive(Deserialize, Clone, Debug)]
pub struct Message {
    /// The unique identifier for this message.
    #[serde(rename = "_id")]
    pub message_id: String,
    /// Unknown purpose.
    #[serde(rename = "outMessage")]
    pub out_message_id: String,
    /// The message text - should be displayed as formatted markdown.
    pub text: String,
    /// The direction the message is going: who sent it.
    ///
    /// For [`ChannelUpdate::UserMessage`] messages, this will always be `Incoming`.
    /// For [`ChannelUpdate::UserConversation`] updates, this can be both `Incoming` for new messages,
    /// or `Outgoing` for messages sent by either this client or another client logged in as the same user.
    ///
    /// [`ChannelUpdate::UserMessage`]: ../enum.ChannelUpdate.html
    /// [`ChannelUpdate::UserConversation`]: ../enum.ChannelUpdate.html
    #[serde(rename = "type")]
    pub direction: MessageDirectionType,
    /// Whether or not the user who received this message has read it... Probably going to be false, as this is a
    /// message that was just sent.
    pub unread: bool,
    /// The user who is subscribed to the channel and either received or sent this message.
    #[serde(rename = "user")]
    pub user_id: String,
    /// The other user involved in this conversation: the one who isn't the user who received this update.
    #[serde(rename = "respondent")]
    pub respondent_id: String,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Update for a newly received message.
#[derive(Deserialize, Clone, Debug)]
pub struct MessageUpdate {
    /// The message.
    pub message: Message,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Update on whether a message is unread or not.
#[derive(Deserialize, Clone, Debug)]
pub struct MessageUnreadUpdate {
    /// The unique identifier for this message.
    #[serde(rename = "_id")]
    pub message_id: String,
    /// Whether or not it is now unread. Most likely `false`: going from read to unread is not supported in screeps
    /// as of this writing.
    pub unread: bool,
    /// Phantom data in order to allow adding any additional fields in the future.
    #[serde(skip)]
    _phantom: PhantomData<()>,
}

/// Update on a conversation between two specific users. This is either a new message sent by one of the users
/// (either the subscribed one or the other one), or an update indicating that a message previously sent has now
/// been read.
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConversationUpdate {
    /// A new message has been sent.
    NewMessage { message: Message },
    /// A message's `unread` status has changed.
    MessageRead { message: MessageUnreadUpdate },
}
