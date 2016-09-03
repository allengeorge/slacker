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

// TODO: encode emoji
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer, Visitor};
use serde_json;
use std::fmt;
use url::form_urlencoded;

use base_types::SlackUrl;
use errors::Result as SlackResult;
use errors::ErrorKind;
use serde_types::{Color, LinkNames, Message, MessageParseBehavior};

static LINK_NAMES_ENABLE: &'static str = "1";
static LINK_NAMES_DISABLE: &'static str = "0";

static MESSAGE_PARSE_BEHAVIOR_FULL: &'static str = "full";
static MESSAGE_PARSE_BEHAVIOR_NONE: &'static str = "none";

static SLACK_COLOR_GOOD: &'static str = "good";
static SLACK_COLOR_WARNING: &'static str = "warning";
static SLACK_COLOR_DANGER: &'static str = "danger";

//
// LinkNames
//

// Overridden `fmt::Display` trait for `LinkNames`
// that controls how this type will be translated
// into a JSON string.
impl fmt::Display for LinkNames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simple_name())
    }
}

impl LinkNames {
    fn simple_name(&self) -> &str {
        match *self {
           LinkNames::Enable => LINK_NAMES_ENABLE,
           LinkNames::Disable => LINK_NAMES_DISABLE,
        }
    }
}

//
// MessageParseBehavior
//

// Overridden `fmt::Display` trait for `MessageParseBehavior`
// that controls how this type will be translated
// into a JSON string.
impl fmt::Display for MessageParseBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simple_name())
    }
}

impl MessageParseBehavior {
    fn simple_name(&self) -> &str {
        match *self {
           MessageParseBehavior::Full => MESSAGE_PARSE_BEHAVIOR_FULL,
           MessageParseBehavior::None => MESSAGE_PARSE_BEHAVIOR_NONE,
        }
    }
}

//
// Color
//

// Overridden `fmt::Display` trait for `Color`
// that controls how this type will be translated
// into a JSON string.
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color_to_string())
    }

}

impl Color {
    fn color_to_string(&self) -> &str {
        match *self {
            Color::Good => SLACK_COLOR_GOOD,
            Color::Warning => SLACK_COLOR_WARNING,
            Color::Danger => SLACK_COLOR_DANGER,
            Color::Hex(ref contained) => contained,
        }
    }
}

//
// SlackUrl
//

impl Serialize for SlackUrl {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let &SlackUrl(ref url) = self;
        serializer.serialize_str(url.as_str())
    }
}

struct SlackUrlVisitor;
impl Visitor for SlackUrlVisitor {
    type Value = SlackUrl;
}

impl Deserialize for SlackUrl {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let v = SlackUrlVisitor {};
        deserializer.deserialize_str(v)
    }
}

//
// message building
//

macro_rules! encode_field {
    ($i:expr, $f:ident, $o:expr) => {
        $i.$f.as_ref().map(|f| $o.append_pair(stringify!($f), &f.to_string()))
    };
}

/// Encodes a `Message` struct into a series of key-value pairs
/// to be sent to slack.
/// NOTE: if you want your message to contain a literal '&', '>' or '<',
/// entity-encode it before setting `Message.text`.
pub fn encode_message(message: &Message) -> SlackResult<String> {
    let mut encoded = form_urlencoded::Serializer::new(String::new());

    let has_text = message.text.is_some();
    let has_attachments = message.attachments.is_some();

    if !has_text && !has_attachments {
        return Err(ErrorKind::NoMessageContent.into());
    }

    if has_text {
        let given_text = message.text.as_ref().unwrap();
        let encoded_text = try!(encode_message_text(&given_text));
        encoded.append_pair("text", &encoded_text);
    }
    if has_attachments {
        let attachments = message.attachments.as_ref().unwrap();
        let serialized = try!(serde_json::to_string(attachments));
        encoded.append_pair("attachments", &serialized);
    }

    encode_field!(message, parse, encoded);
    encode_field!(message, link_names, encoded);
    encode_field!(message, unfurl_links, encoded);
    encode_field!(message, unfurl_media, encoded);
    encode_field!(message, username, encoded);
    encode_field!(message, as_user, encoded);
    encode_field!(message, icon_url, encoded);
    encode_field!(message, icon_emoji, encoded);
    encode_field!(message, mrkdwn, encoded);

    let finished = encoded.finish();
    print!("{:?}", finished);
    Ok(finished)
}

// TODO: only have to entity-encode `&`, `<`, `>`; applies to all text
// TODO: unicode characters have to be turned into space-separated hex and URL encoded
// TODO: unsure how `<`, `>` and `&` control characters play with encoding above
fn encode_message_text(text: &str) -> SlackResult<String> {
    Ok(text.to_owned())
}
