//! Websocket command creation.
use std::str;

use serde_json;

use Token;

use super::Channel;

/// Gets the raw websocket string to send for subscribing to a channel.
///
/// Subscribing to a channel you are already subscribed to may have differing
/// results depending on the server software (official vs. private). Note that if you subscribe
/// multiple times, it may be necessary to unsubscribe at least that many times to fully unsubscribe.
/// Subscribing multiple times may or may not result in duplicated messages, and may or may not
/// result in extra initial messages.
///
/// It's recommended that you keep track of what channels you are subscribed to separately: this
/// is tracked by the server, but is not tracked by `screeps-api`, and cannot be queried from the
/// server.
pub fn subscribe(channel: &Channel) -> String {
    let message = format!("subscribe {}", channel);

    sockjs_send_from_internal(&message)
}

/// Gets the raw websocket string to send for unsubscribing to a channel.
///
/// Unsubscribing from a channel you are not subscribed to appears to have no affect.
///
/// Recommended that you keep track of what channels you are subscribed to separately: this
/// is tracked by the server, but is not tracked by `screeps-api`, and cannot be queried from the
/// server.
pub fn unsubscribe(channel: &Channel) -> String {
    let message = format!("unsubscribe {}", channel);

    sockjs_send_from_internal(&message)
}

/// Authenticates with the given token.
///
/// After doing this, you'll be able to subscribe and unsubscribe to messages. A "auth success" message
/// will happen as a response which returns either this token or a new one.
pub fn authenticate(token: &Token) -> String {
    let message = "auth ".chars().chain(token.chars()).collect::<String>();

    sockjs_send_from_internal(&message)
}

fn sockjs_send_from_internal<T: AsRef<str>>(source: &T) -> String {
    serde_json::to_string(&(source.as_ref(),))
        .expect("serializing a tuple containing a single string can't fail.")
}
