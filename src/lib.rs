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

// required for error_chain
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

mod base_types;
pub use base_types::*;

mod chat;

mod errors;
pub use errors::{Error, ErrorKind, Result};

mod serde_types {
    include!("serde_types.rs");
}
pub use serde_types::{Channel, LinkNames, Message, MessageParseBehavior, Purpose, Topic};

// This is the main touch-point for library users.
mod slack;
pub use slack::Slack;

// TODO: have fixture JSON responses that I have to parse
// TODO: have setup/teardown methods for the test
// TODO: actually have a way of comparing generated message (GET/POST) with expected
// TODO: add a test that checks for attachments
// TODO: add a test that checks message encoding (i.e. &, >, < and emoji)

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    static BOT_API_KEY: &'static str = ""; // FIXME: read from FS
    static USR_API_KEY: &'static str = ""; // FIXME: read from FS

    // TODO: check received arguments
    #[test]
    fn api_test_success() {
        let slack = Slack::new(BOT_API_KEY);
        let mut arguments = HashMap::new();
        arguments.insert("arg1", "foo");
        arguments.insert("arg2", "bar");
        let response = slack.api_test(arguments, None).unwrap();
        print!("{:?}", response)
    }

    // TODO: check received arguments
    #[test]
    fn api_test_error() {
        let slack = Slack::new(BOT_API_KEY);

        let sent_error_string = "foo_error";
        let mut arguments = HashMap::new();
        arguments.insert("arg1", "foo");
        arguments.insert("arg2", "bar");

        let response = slack.api_test(arguments, Some(sent_error_string));
        assert_eq!(response.is_err(), true);

        let received_error_string = match response {
            Err(error) => {
                match *error.kind() {
                    ErrorKind::Unknown(ref error_string) => error_string.clone(),
                    _ => panic!("unknown error kind")
                }
            }
            _ => panic!("api test received no error")
        };
        assert_eq!(received_error_string, sent_error_string);
    }

    #[test]
    fn list_channels() {
        let slack = Slack::new(BOT_API_KEY);
        let channels = slack.channels_list(true).unwrap();
        assert!(channels.len() >= 1) // must contain at least the general channel
    }

//    #[test]
//    fn join_channel() {
//        let slack = Slack::new(USR_API_KEY);
//        let response = slack.channels_join(&"test-0".to_string()).unwrap();
//        assert_eq!(response, ());
//    }
//
//    #[test]
//    fn leave_channel() {
//        let slack = Slack::new(USR_API_KEY);
//        let response = slack.channels_leave(&"C272LM3HT".to_string()).unwrap();
//        assert_eq!(response, ());
//    }

    #[test]
    fn post_message() {
        let slack = Slack::new(BOT_API_KEY);
        let params = Message
            {
                text: Some("this & < & >".to_owned()),
                parse: Some(MessageParseBehavior::Full),
                link_names: Some(LinkNames::Enable),
                .. Default::default()
            };
        let response = slack.chat_post_message(&"#general".to_string(), &params).unwrap();
        assert_eq!(response, ());
    }

    #[test]
    fn send_message_with_link() {
        let slack = Slack::new(BOT_API_KEY);
        let params = Message
            {
                text: Some("<http://www.google.com>".to_owned()),
                .. Default::default()
            };
        let response = slack.chat_post_message(&"#general".to_string(), &params).unwrap();
        assert_eq!(response, ());
    }

    #[test]
    fn send_message_with_unicode() {
        let slack = Slack::new(BOT_API_KEY);
        let params = Message
            {
                text: Some("α".to_owned()),
                .. Default::default()
            };
        let response = slack.chat_post_message(&"#general".to_string(), &params).unwrap();
        assert_eq!(response, ());
    }

    #[test]
    fn send_message_with_emoji() {
        let slack = Slack::new(BOT_API_KEY);
        let params = Message
            {
                text: Some("😁 😳".to_owned()),
                .. Default::default()
            };
        let response = slack.chat_post_message(&"#general".to_string(), &params).unwrap();
        assert_eq!(response, ());
    }
}
