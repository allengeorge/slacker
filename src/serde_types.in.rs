// Copyright 2016 Allen A. George.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_json;

use base_types::{ChannelId, SlackUrl, UserId};

// TODO: does serde work with default trait
// TODO: does serde work with extends? (i.e. a struct that implements a trait?)

//
// API response definitions
//

/// Actual response received from an api.test response.
///
/// See [Slack api.test Method (Response)](https://api.slack.com/methods/api.test "Slack api.test Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiTestResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error, or, if the caller specified the "error" parameter.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// Arguments supplied in the api.test call.
    pub args: Option<serde_json::Value>,
}

/// Actual response received from an channels.archive response.
///
/// See [Slack channels.archive Method (Response)](https://api.slack.com/methods/channels.archive "Slack channels.archive Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsArchiveResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error, or, if the caller specified the "error" parameter.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
}

/// Actual response received from an channels.create response.
///
/// See [Slack channels.create Method (Response)](https://api.slack.com/methods/channels.create "Slack channels.create Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsCreateResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error, or, if the caller specified the "error" parameter.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// A `Channel` object representing the channel created by the `channels.create` call.
    pub channel: Option<Channel>,
}

/// Actual response received from an channels.info response.
///
/// See [Slack channels.info Method (Response)](https://api.slack.com/methods/channels.info "Slack channels.info Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsInfoResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error, or, if the caller specified the "error" parameter.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// A `Channel` object describing the requested channel.
    pub channel: Option<Channel>,
}

/// Actual response received from an channels.invite response.
///
/// See [Slack channels.invite Method (Response)](https://api.slack.com/methods/channels.invite "Slack channels.invite Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsInviteResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error, or, if the caller specified the "error" parameter.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// A `Channel` object describing the state of the channel after the invite succeeds.
    pub channel: Option<Channel>,
}

/// Actual response received from a channels.join call.
///
/// See [Slack channels.join Method (Response)](https://api.slack.com/methods/channels.join "Slack channels.join Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsJoinResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// Only populated if the user has already joined the channel.
    pub already_in_channel: Option<bool>,
    /// If successful, the channel joined.
    pub channel: Option<Channel>,
}

/// Actual response received from a channels.leave call.
///
/// See [Slack channels.leave Method (Response)](https://api.slack.com/methods/channels.leave "Slack channels.leave Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsLeaveResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// Only populated if the user was not in the channel.
    pub not_in_channel: Option<bool>,
}

/// Actual response received from a channels.list call.
///
/// See [Slack channels.list Method (Response)](https://api.slack.com/methods/channels.list "Slack channels.list Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsListResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// List of channels in this team.
    pub channels: Vec<Channel>, // there'll be at least one: #general
}

/// Actual response received from a channels.mark call.
///
/// See [Slack channels.mark Method (Response)](https://api.slack.com/methods/channels.mark "Slack channels.mark Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsMarkResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
}

/// Actual response received from a channels.rename call.
///
/// See [Slack channels.rename Method (Response)](https://api.slack.com/methods/channels.rename "Slack channels.rename Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsRenameResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// New (limited) state of the channel.
    pub channel: Option<RenamedChannel>,
}

///  Limited representation of a channel after a `channels.rename` call.
///
/// See [Slack channels.rename Method (Response)](https://api.slack.com/methods/channels.rename "Slack channels.rename Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct RenamedChannel {
    /// Unique ID of the channel.
    pub id: ChannelId,
    /// `true` if this is a channel, `false` if it's a DM or a group chat.
    pub is_channel: bool,
    /// Human-readable channel name.
    pub name: String,
    /// Time at which the channel was created.
    pub created: u32,
}

/// Actual response received from a channels.setPurpose call.
///
/// See [Slack channels.setPurpose Method (Response)](https://api.slack.com/methods/channels.setPurpose "Slack channels.setPurpose Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsSetPurposeResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// New purpose for this channel.
    pub purpose: Option<String>,
}

/// Actual response received from a channels.setTopic call.
///
/// See [Slack channels.setTopic Method (Response)](https://api.slack.com/methods/channels.setTopic "Slack channels.setTopic Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsSetTopicResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// New topic for this channel.
    pub topic: Option<String>,
}

/// Actual response received from a channels.unarchive call.
///
/// See [Slack channels.unarchive Method (Response)](https://api.slack.com/methods/channels.unarchive "Slack channels.unarchive Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelsUnarchiveResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
}

/// Actual response received from a chat.delete call.
///
/// See [Slack chat.delete Method (Response)](https://api.slack.com/methods/chat.delete "Slack chat.delete Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatDeleteResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// Channel from which the message was deleted if the request was successful.
    pub channel: Option<ChannelId>,
    /// Timestamp of the deleted message if the request was successful.
    pub ts: Option<f64>,
}

/// Actual response received from a chat.meMessage call.
///
/// See [Slack chat.meMessage Method (Response)](https://api.slack.com/methods/chat.meMessage "Slack chat.meMessage Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMeMessageResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    /// Channel to which the me-message was posted if the request was successful.
    pub channel: Option<ChannelId>,
    /// Timestamp of the me-message if the request was successful.
    pub ts: Option<f64>,
}

// Actual response received from a chat.postMessage call.
///
/// See [Slack chat.postMessage Method (Response)](https://api.slack.com/methods/chat.postMessage "Slack chat.postMessage Method (Response)")
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPostMessageResponse {
    /// `true` if the request was successful, `false` otherwise.
    pub ok: bool,
    /// Only populated if there is an error.
    pub error: Option<String>,
    /// Only populated if there is a warning.
    pub warning: Option<String>,
    // Time when the message was posted to the channel.
    pub ts: Option<f64>,
    // Channel to which the message was posted.
    pub channel: Option<ChannelId>,
    // Final message content as posted to the channel.
    pub message: Option<Message>,
}

//
// Object definitions
//

/// Message to be posted via `chat.postMessage`.
///
/// See [Slack chat.postMessage Method (Request))](https://api.slack.com/methods/chat.postMessage "Slack chat.postMessage Method (Request)")
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    /// Message text. Required, unless `attachments` are provided.
    /// You may use both `text` and `attachments`.
    pub text: Option<String>,
    /// Attachments to be included in the message
    pub attachments: Option<Vec<Attachment>>,
    /// Set message parsing behavior.
    ///
    /// * If set to `Some(full)` Slack treats the incoming message
    /// as unformatted text and finds and linkifies URLs, usernames
    /// and channels.
    /// * If set to `Some(none)` Slack does *not* process the
    /// incoming message beyond checking basic validity.
    /// * If set to `None` non-linked URLs are linkified,
    /// but usernames and channels are not (this can be turned
    /// on by setting `link_names` below).
    pub parse: Option<MessageParseBehavior>,
    /// Set to `1` if user and channel names
    /// should automatically be linked within the message.
    pub link_names: Option<LinkNames>,
    /// Create an attachment for each text-based link in the message.
    pub unfurl_links: Option<bool>,
    /// Create an attachment for each media link in the message.
    pub unfurl_media: Option<bool>,
    /// Set the bot username.
    pub username: Option<String>,
    /// Post the message as the user specified in `username`.
    /// If this value is `false` then the user posting the message
    /// will be inferred by the server.
    pub as_user: Option<bool>,
    /// URL to an image to use as the icon for this message. Must be used with
    /// `as_user` set to `false`, otherwise ignored.
    pub icon_url: Option<SlackUrl>,
    // Emoji to use as the icon for this message. Overrides `icon_url`. Must be
    // used with `as_user` set to `false`, otherwise ignored.
    pub icon_emoji: Option<String>,
    /// Set to `true` to enable formatting of a message sent by
    /// a bot, `false` otherwise. The default is `true`, so message
    /// text in a bot-sent message is always markdown-formatted.
    pub mrkdwn: Option<bool>,
}

/// Control how the Slack server will parse
/// and auto-link messages sent by this client.
///
/// See [Basic message formatting (Parsing modes)](https://api.slack.com/docs/message-formatting#linking_to_urls "Basic message formatting (Parsing modes)")
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageParseBehavior {
    /// Parse the supplied message and auto-link
    /// any user names (identified by a preceding `@`)
    /// channel names (identified by a preceding `#`),
    /// or URLs.
    Full,
    /// Do not perform any parsing on the supplied message.
    /// User and channel names as well as links will not be
    /// linkified, even if present.
    None,
}

/// Control how the Slack server will parse
/// and auto-link channel names and user names.
///
/// The server *always* ignores this value and
/// always assumes `LinkNames::Enable` if `MessageParseBehavior::Full`
/// is sent.
///
/// See [Basic message formatting (Parsing modes)](https://api.slack.com/docs/message-formatting#linking_to_urls "Basic message formatting (Parsing modes)")
#[derive(Serialize, Deserialize, Debug)]
pub enum LinkNames {
    /// Enable auto-linking channel and user names in a message.
    Enable,
    /// Disable auto-linking channel and user names in a message.
    Disable,
}

/// Represents an attachment in a Slack message.
/// All fields in this struct are exact analogs
/// of the Slack Web API JSON.
///
/// See [Attaching content and links to messages](https://api.slack.com/docs/message-attachments "Attaching content and links to messages")
#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    /// Plain-text summary of the attachment.
    pub fallback: String,
    /// Attachment color.
    /// Hex-color code or "good", "warning", "danger".
    pub color: Option<Color>,
    /// Optional text that appears above the attachment block.
    pub pretext: Option<String>,
    /// Name of the person that created the attachment.
    pub author_name: Option<String>,
    /// Link to the author's webpage. This will hyperlink the author's
    /// name, and will only take effect if `author_name` is defined.
    pub author_link: Option<SlackUrl>,
    /// Link to a small 16x16px image to the left of the `author_name`
    /// text. This only takes effect if `author_name` is defined.
    pub author_icon: Option<SlackUrl>,
    /// Larger, bold text on top of the message attachment.
    pub title: Option<String>,
    /// Link that will be used to hyperlink `title` if it exists.
    pub title_link: Option<SlackUrl>,
    /// Main text in the attachment. It can contain standard message markup. The
    /// content automatically collapses if it contains 700+ characters or 5+ linebreaks,
    /// and will display a "Show more..." link to expand the content.
    pub text: Option<String>,
    /// Fields to display in a table in the attachment.
    /// Fields have a title and a value.
    pub fields: Option<Vec<AttachmentFields>>,
    /// URL to an image to be displayed inside a message attachment.
    /// Large images will be resized to a maximum width of 400px or a
    /// maximum height of 500px while maintaining the same aspect ratio.
    pub image_url: Option<SlackUrl>,
    /// URL to an image that will be displayed as a thumbnail to the right
    /// of a message attachment. The thumbnail's largest dimension will be
    /// resided to a maximum of 75px while maintaining the aspect ratio.
    pub thumb_url: Option<SlackUrl>,
    /// Text at the bottom of the attachment. Limited to 300 characters and
    /// may be truncated for users with smaller displays.
    pub footer: Option<String>,
    /// Url to an image to be displayed next to the footer text. Will be
    /// resized to 16px by 16px. Only takes effect if `footer` is defined.
    pub footer_icon: Option<SlackUrl>,
    /// Epoch time associated with this attachment. Used to indicate that
    /// the attachment refers to an event as a specific time, and will
    /// be rendered in a human-readable format.
    pub ts: u32,
    /// List of attachment fields (`text`, `pretext`, etc.) to be formatted
    /// using markdown in bot-sent messages. By default, attachment fields
    /// in bot-sent messages are *not* markdown-formatted.
    pub mrkdwn_in: Option<Vec<String>>,
}

/// Color with which to highlight a message attachment.
///
/// See [Attaching content and links to messages](https://api.slack.com/docs/message-attachments "Attaching content and links to messages")
#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    /// Indicates a normal-priority message.
    Good,
    /// Indicates a warning message.
    Warning,
    /// Indicates a high-priority message.
    Danger,
    /// Custom RGB hex color code.
    Hex(String) // TODO: can I check this at compile time?
}

/// Attachment fields that can be displayed in an `Attachment`.
///
/// See [Attaching content and links to messages](https://api.slack.com/docs/message-attachments "Attaching content and links to messages")
#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentFields {
    /// Bold heading above the value text. This cannot
    /// contain markup and will be automatically escaped.
    pub title: String,
    /// Field value.
    pub value: String,
    /// `true` if the `value` is short enough to be
    /// displayed side-by-side with other values.
    pub short: bool,
}

/// Represents a single Slack channel.
/// All fields in this struct are exact analogs of
/// the Slack Web API JSON.
///
/// The first six fields are mandatory, with the following
/// fields optional. An object containing only the mandatory
/// fields is considered a "limited channel object".
/// See [Slack channels.join Method (Response)](https://api.slack.com/methods/channels.join "Slack channels.join Method (Response)")
///
/// See [Slack Object Types: Channel](https://api.slack.com/types/channel "Slack Object Types: Channel")
#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    /// Unique ID of the channel.
    pub id: ChannelId,
    /// Human-readable channel name.
    pub name: String,
    /// Time at which the channel was created.
    pub created: u32,
    /// Unique ID of the user that created the channel.
    pub creator: UserId,
    /// `true` if the channel has been archived and is no longer active, `false` otherwise.
    pub is_archived: bool,
    /// `true` if the channel is `#general`, `false` otherwise.
    pub is_general: bool,
    /// User ids for all users in this channel, including disabled
    /// users that were in this channel when they were disabled.
    pub members: Option<Vec<UserId>>,
    /// Channel topic.
    pub topic: Option<Topic>,
    /// Channel purpose.
    pub purpose: Option<Purpose>,
    /// `true` if the user making the API call is a member of this channel,
    /// `false` otherwise.
    pub is_member: Option<bool>,
    /// Timestamp of the last message the user making the API call read.
    pub last_read: Option<f64>,
    /// Count of all visible messages that the user making the API call has yet to read.
    pub unread_count: Option<u32>,
    /// Count of messages that the user making the API call
    /// has yet to read. This excludes join/leave messages.
    pub unread_count_display: Option<u32>,
}

/// Represents the topic of a Slack channel.
/// All fields in this struct are exact analogs of
/// the Slack Web API JSON.
///
/// See [Slack Object Types: Channel](https://api.slack.com/types/channel "Slack Object Types: Channel")
#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    /// Topic text.
    value: String,
    /// User who set the topic.
    creator: UserId,
    /// Epoch time at which the topic was last set.
    last_set: u32,
}

/// Represents the purpose of a Slack channel.
/// All fields in this struct are exact analogs of
/// the Slack Web API JSON.
///
/// See [Slack Object Types: Channel](https://api.slack.com/types/channel "Slack Object Types: Channel")
#[derive(Serialize, Deserialize, Debug)]
pub struct Purpose {
    /// Purpose text.
    value: String,
    /// User who set the purpose.
    creator: UserId,
    /// Epoch time at which the purpose was last set.
    last_set: u32,
}
