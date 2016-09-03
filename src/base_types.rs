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

use hyper;
use std::ops::Deref;

// TODO: verify ChannelId
// TODO: verify UserId

/// Type alias for a unique Slack channel id.
/// (Note that this is *not* the channel's display name.)
pub type ChannelId = String; // TODO: should this be an enum?

/// Type alias for a unique Slack user id.
/// (Note that this is *not* the user's display name.)
pub type UserId = String;

/// Wrapper over hyper::Url.
///
/// Implementation note: Done because
/// we can't implement a trait for a type
/// where both the trait and type are
/// defined in external crates.
#[derive(Debug)]
pub struct SlackUrl(pub hyper::Url);

impl Deref for SlackUrl {
    type Target = hyper::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
