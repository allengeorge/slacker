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
use serde_json;
use std::io;
use url;

error_chain! {
    foreign_links {
        // Error from the HTTP library.
        hyper::Error, HttpError;
        // Error while making Slack API call.
        io::Error, IoError;
        // Unable to parse Slack API response.
        serde_json::Error, JsonParseError;
        // Unable to parse Slack API endpoint URL.
        url::ParseError, UrlParseError;
    }

    // TODO: get failing api method names or bad arg names/values where possible
    errors {
        /// No auth token provided.
        NoAuthToken {
            description("no auth token")
            display("no auth token provided in slack api method call")
        }
        /// Invalid auth token provided.
        InvalidAuthToken {
            description("invalid auth token")
            display("invalid auth token provided in slack api method call")
        }
        /// Auth token provided was for a deleted user or team.
        InactiveAuthToken {
            description("inactive auth token")
            display("auth token for deleted user or team provided in slack api method call")
        }
        /// Attempted to call an API method unavailable to bot users.
        MethodForbiddenForBots {
            description("slack api method cannot be called by bots")
            display("slack api method cannot be called by bots")
        }
        /// Attempted to call an API method unavailable to restricted users or single channel guests.
        MethodForbiddenForRestrictedUser {
            description("slack api method cannot be called by restricted users")
            display("slack api method cannot be called by restricted users")
        }
        /// Attempted to call an API method unavailable to single channel guests.
        MethodForbiddenForSingleChannelGuest {
            description("slack api method cannot be called by a single channel guest")
            display("slack api method cannot be called by a single channel guest")
        }
        /// User specified is invalid or cannot be found.
        UserNotFound {
            description("user not found")
            display("slack api method specified an invalid user")
        }
        /// Invalid Slack channel id.
        InvalidChannelId {
            description("invalid slack channel id")
            display("slack channel id is missing initial identifier or is malformed")
        }
        /// Invalid Slack user id.
        InvalidUserId {
            description("invalid slack user id")
            display("slack user id is missing initial identifier or is malformed")
        }
        /// Argument name is too long or contains invalid characters.
        InvalidMethodArg {
            description("slack api method argument is malformed")
            display("slack api method argument is too long or contains invalid characters")
        }
        /// Method was passed an array.
        InvalidArrayArg {
            description("slack api method argument has invalid value")
            display("slack api method non-array argument has an array value")
        }
        /// Charset passed in the API call was neither `utf-8` or `iso-8859-1`.
        InvalidCharset {
            description("slack api method called with an invalid charset")
            display("slack api method call specifies a charset other than utf-8 or iso-8859-1")
        }
        /// API call was made via POST, but form data was missing or invalid.
        InvalidFormData {
            description("slack api method uses POST with invalid form data")
            display("slack api method made using POST but the form data was missing or invalid")
        }
        /// Content-type in API POST call was not `application/json`,
        /// `application/x-www-form-urlencoded`, `multipart/form-data`
        /// or `text/plain`.
        InvalidPostType {
            description("slack api method uses POST with an unsupported content-type")
            display("slack api method made using POST with an unsupported content type header")
        }
        /// Content-type in API POST call was not set.
        MissingPostType {
            description("slack api method uses POST with missing content-type")
            display("slack api method made using POST with a missing content-type header")
        }
        /// API POST call content was missing or truncated.
        RequestTimeout {
            description("slack api method failed with a timeout")
            display("slack api method call made using POST without content or with truncated content")
        }
        /// A team setting prevents the user making the request from creating channels.
        ChannelActionRestricted {
            description("user cannot create channels")
            display("team settings prevent user from creating channels")
        }
        /// No channel specified in channel request.
        NoChannel {
            description("slack api method call missing channel argument")
            display("slack api method call missing channel argument")
        }
        /// Channel cannot be created with the requested name.
        ChannelNameTaken {
            description("channel cannot be created with requested name")
            display("channel cannot be created with requested name")
        }
        /// Message could not be posted because the user does not belong to the destination channel.
        NotInChannel {
            description("user cannot post message to channel")
            display("cannot post message because the user not a member of channel")
        }
        /// Destination channel is archived and inactive.
        ChannelIsArchived {
            description("user cannot post message to archived channel")
            display("cannot post message because the channel is archived")
        }
        /// Cannot archive the general channel.
        CannotArchiveGeneralChannel {
            description("cannot archive the general channel")
            display("cannot archive the '#general' channel")
        }
        /// Cannot invite self to a channel.
        CannotInviteSelfToChannel {
            description("cannot invite self")
            display("cannot invite the caller of the api method to the requested channel")
        }
        /// Invited user is already a member of the channel.
        UserAlreadyMemberOfChannel {
            description("user already member of channel")
            display("invited user is already a member of the channel")
        }
        // Cannot invite the user to the channel.
        CannotInviteUserToChannel {
            description("cannot invite user")
            display("cannot invite the user to the requested channel")
        }
        // Too many users invited at once in a single API call.
        TooManyUsersInvitedAtOnce {
            description("too many users invited at once")
            display("invited more than 30 users to the channel in a single slack api method call")
        }
        /// Cannot archive last channel for a multi-channel guest
        CannotArchiveLastRestrictedActionChannel {
            description("cannot archive last channel for multi-channel guest")
            display("cannot archive last channel for multi-channel guest")
        }
        /// Invalid channel specified.
        ChannelNotFound {
            description("user cannot post message because channel is invalid")
            display("cannot post message because channel is invalid")
        }
        /// Authenticated user cannot leave the `#general` channel.
        CannotLeaveGeneralChannel {
            description("cannot leave the '#general' channel")
            display("cannot leave the '#general' channel")
        }
        /// Invalid timestamp passed in to slack API method call.
        InvalidTimestamp {
            description("invalid timestamp")
            display("invalid timestamp passed to slack API method")
        }
        /// Channel purpose or topic exceeded 250 characters.
        ChannelPurposeOrTopicTooLong {
            description("purpose or topic too long")
            display("channel purpose or topic exceeded 250 characters")
        }
        /// Channel is not archived and so, cannot be unarchived.
        ChannelNotArchived {
            description("channel not archived")
            display("channel not archived, so cannot be unarchived")
        }
        /// Message to be deleted/modified cannot be found.
        MessageNotFound {
            description("message not found")
            display("message to be modified or deleted cannot be found")
        }
        /// Compliance exports are enabled, preventing message deletion.
        /// User does not have permissions to delete the message.
        UserCannotDeleteMessage {
            description("user cannot delete message")
            display("user does not have permissions to delete the message")
        }
        /// Compliance exports are enabled, preventing message deletion.
        ComplianceExportsPreventDeletion {
            description("compliance exports prevent message deletion")
            display("compliance exports are enabled, and prevent message deletion")
        }
        /// No message text.
        NoMessageContent {
            description("no message content")
            display("attempting to post message with no attachments and no text")
        }
        /// Message text is too long.
        MessageTooLong {
            description("slack message text too long")
            display("cannot post message because text exceeds limit")
        }
        /// A call to chat.postMessage was made, but no "text" field was submitted.
        MessageHasNoText {
            description("slack message has no content")
            display("cannot post message because it has no content")
        }
        /// Too many attachments for this message.
        MessageHasTooManyAttachments {
            description("slack message has too many attachments")
            display("cannot post message because it has too many attachments")
        }
        /// Too many messages posted.
        RateLimited {
            description("rate-limited because too many messages posted")
            display("cannot post message because message-posting has been rate-limited")
        }
        /// Unknown error returned from a slack API method call.
        Unknown(error_string: String) {
            description("slack api method returned unknown error") // FIXME: apparently description cannot be formatted
            display("slack api method returned unknown error '{}'", error_string)
        }
    }
}

pub fn from_api_error_string(error_string: &String) -> Error {
    let kind = match error_string.as_str() {
        "not_authed" => ErrorKind::NoAuthToken,
        "invalid_auth" => ErrorKind::InvalidAuthToken,
        "account_inactive" => ErrorKind::InactiveAuthToken,
        "user_is_bot" => ErrorKind::MethodForbiddenForBots,
        "user_is_restricted" => ErrorKind::MethodForbiddenForRestrictedUser,
        "user_is_ultra_restricted" => ErrorKind::MethodForbiddenForSingleChannelGuest,
        "user_not_found" => ErrorKind::UserNotFound,
        "invalid_arg_name" => ErrorKind::InvalidMethodArg,
        "invalid_array_arg" => ErrorKind::InvalidArrayArg,
        "invalid_charset" => ErrorKind::InvalidCharset,
        "invalid_form_data" => ErrorKind::InvalidFormData,
        "invalid_post_type" => ErrorKind::InvalidPostType,
        "missing_post_type" => ErrorKind::MissingPostType,
        "request_timeout" => ErrorKind::RequestTimeout,
        "restricted_action" => ErrorKind::ChannelActionRestricted,
        "no_channel" => ErrorKind::NoChannel,
        "name_taken" => ErrorKind::ChannelNameTaken,
        "not_in_channel" => ErrorKind::NotInChannel,
        "is_archived" => ErrorKind::ChannelIsArchived,
        "already_archived" => ErrorKind::ChannelIsArchived,
        "cant_archive_general" => ErrorKind::CannotArchiveGeneralChannel,
        "cant_invite_self" => ErrorKind::CannotInviteSelfToChannel,
        "already_in_channel" => ErrorKind::UserAlreadyMemberOfChannel,
        "cant_invite" => ErrorKind::CannotInviteUserToChannel,
        "too_many_users" => ErrorKind::TooManyUsersInvitedAtOnce,
        "last_ra_channel" => ErrorKind::CannotArchiveLastRestrictedActionChannel,
        "channel_not_found" => ErrorKind::ChannelNotFound,
        "cant_leave_general" => ErrorKind::CannotLeaveGeneralChannel,
        "invalid_timestamp" => ErrorKind::InvalidTimestamp,
        "too_long" => ErrorKind::ChannelPurposeOrTopicTooLong,
        "not_archived" => ErrorKind::ChannelNotArchived,
        "message_not_found" => ErrorKind::MessageNotFound,
        "cant_delete_message" => ErrorKind::UserCannotDeleteMessage,
        "compliance_exports_prevent_deletion" => ErrorKind::ComplianceExportsPreventDeletion,
        "msg_too_long" => ErrorKind::MessageTooLong,
        "no_text" => ErrorKind::MessageHasNoText,
        "too_many_attachments" => ErrorKind::MessageHasTooManyAttachments,
        "rate_limited" => ErrorKind::RateLimited,
        _ => ErrorKind::Unknown(error_string.clone()),
    };

    kind.into()
}
