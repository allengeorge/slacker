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

use std::collections::HashMap;
use std::convert::From;
use std::io::Read;

use hyper::client::{Client, RequestBuilder};
use hyper::header::{ContentLength, ContentType};
use hyper::client::response::Response;
use hyper::Url;

use base_types::{ChannelId, UserId};
use chat;
use errors::*;
use serde_types::*;

use serde::Deserialize;
use serde_json;

static SLACK_BASE_API_URL: &'static str = "https://slack.com/api";
const DEFAULT_RESPONSE_CONTENT_LENGTH: usize = 256;

macro_rules! slack_result {
    ($x:expr) => {
        if $x.ok {
            Ok($x)
        } else {
            Err(from_api_error_string(&$x.error.unwrap()))
        }
    };
    ($x:expr, ()) => {
        if $x.ok {
            Ok(())
        } else {
            Err(from_api_error_string(&$x.error.unwrap()))
        }
    };
}

pub struct Slack {
    access_token: String,
    client: Client,
}

// TODO: split groups of slack calls into different files
// TODO: expose two versions of the send/recv API: one that returns a message type and another that returns a JsonValue

impl Slack {

    pub fn new(access_token: &str) -> Slack {
        Slack {
            access_token: access_token.to_string(),
            client: Client::new(),
        }
    }

    //
    // api
    //

    // TODO: allow the value in the map to be any string-able type
    pub fn api_test(&self, arguments: HashMap<&str, &str>, error: Option<&str>) -> Result<ApiTestResponse> {
        let mut api_url = try!(self.api_url("api.test"));
        api_url.query_pairs_mut().extend_pairs(arguments.iter());
        error.map(|e| {api_url.query_pairs_mut().append_pair("error", e); () });
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ApiTestResponse>(request));
        slack_result!(deserialized)
    }

    //
    // channels
    //

    pub fn channels_archive(&self, channel: &ChannelId) -> Result<()> {
        let mut api_url = try!(self.api_url("channels.archive"));
        api_url.query_pairs_mut().append_pair("channel", channel);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsArchiveResponse>(request));
        slack_result!(deserialized, ())
    }

    pub fn channels_create(&self, channel_name: &str) -> Result<Channel> {
        let mut api_url = try!(self.api_url("channels.create"));
        api_url.query_pairs_mut().append_pair("name", channel_name);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsCreateResponse>(request));
        slack_result!(deserialized).map(|d| d.channel.unwrap())
    }

    pub fn channels_info(&self, channel: &ChannelId) -> Result<Channel> {
        let mut api_url = try!(self.api_url("channels.info"));
        api_url.query_pairs_mut().append_pair("channel", channel);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsInfoResponse>(request));
        slack_result!(deserialized).map(|d| d.channel.unwrap())
    }

    pub fn channels_invite(&self, channel: &ChannelId, user: &UserId) -> Result<Channel> {
        let mut api_url = try!(self.api_url("channels.invite"));
        api_url.query_pairs_mut().append_pair("channel", channel);
        api_url.query_pairs_mut().append_pair("user", user);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsInviteResponse>(request));
        slack_result!(deserialized).map(|d| d.channel.unwrap())
    }

    pub fn channels_join(&self, channel_name: &str) -> Result<()> {
        let mut api_url = try!(self.api_url("channels.join"));
        api_url.query_pairs_mut().append_pair("name", channel_name);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsJoinResponse>(request));
        slack_result!(deserialized, ())
    }

    pub fn channels_kick(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn channels_leave(&self, channel: &ChannelId) -> Result<()> {
        let mut api_url = try!(self.api_url("channels.leave"));
        api_url.query_pairs_mut().append_pair("channel", channel);
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsLeaveResponse>(request));
        slack_result!(deserialized, ())
    }

    pub fn channels_list(&self, exclude_archived: bool) -> Result<Vec<Channel>> {
        let mut api_url = try!(self.api_url("channels.list"));
        api_url.query_pairs_mut().append_pair("exclude_archived", &exclude_archived.to_string());
        let api_url = api_url;

        let request = self.client.get(api_url);
        let deserialized = try!(Slack::send::<ChannelsListResponse>(request));
        slack_result!(deserialized).map(|d| d.channels)
    }

    pub fn channels_mark(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn channels_rename(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn channels_set_purpose(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn channels_set_topic(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn channels_unarchive(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // chat
    //

    pub fn chat_delete(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn chat_me_message(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn chat_post_message(&self, channel: &ChannelId, message: &Message) -> Result<()> {
        let mut api_url = try!(self.api_url("chat.postMessage"));
        api_url.query_pairs_mut().append_pair("channel", channel);
        let api_url = api_url;

        let message_string = try!(chat::encode_message(message));
        let request = self.client // interesting: can't build request body without url
            .post(api_url)
            .header(ContentType::form_url_encoded())
            .body(&message_string);

        let deserialized = try!(Slack::send::<ChatPostMessageResponse>(request));
        slack_result!(deserialized, ())
    }

    pub fn chat_update(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // reminders
    //

    pub fn reminders_add(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn reminders_complete(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn reminders_delete(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn reminders_info(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn reminders_list(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // search
    //

    pub fn search_all(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn search_files(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn search_messages(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // users
    //

    pub fn users_delete_photo(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_get_presence(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_identity(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_info(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_list(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_set_active(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_set_photo(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_set_presence(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // user profile
    //

    pub fn users_profile_get(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn users_profile_set(&self) -> Result<()> {
        unimplemented!()
    }

    //
    // catch-all send api
    //

    pub fn api_url(&self, slack_method: &str) -> Result<Url> {
        let mut url = try!(Url::parse(&format!("{}/{}", SLACK_BASE_API_URL, slack_method)));
        url.query_pairs_mut().append_pair("token", &self.access_token);
        Ok(url)
    }

    pub fn send<T>(request: RequestBuilder) -> Result<T> where T: Deserialize {
        request.send().map_err(From::from).and_then(|mut r| Slack::deserialize::<T>(&mut r))
    }

    //
    // helpers
    //

    fn deserialize<T>(response: &mut Response) -> Result<T> where T: Deserialize {
        let content_length = response.headers
            .get::<ContentLength>()
            .map_or(DEFAULT_RESPONSE_CONTENT_LENGTH, |c| { let ContentLength(length) = *c; length as usize });
        let mut body = String::with_capacity(content_length);
        try!(response.read_to_string(&mut body));
        serde_json::from_str::<T>(&body).map_err(From::from)
    }
}
