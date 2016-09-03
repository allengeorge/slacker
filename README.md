# slacker

## Next steps
* Properly serialize messages
* Properly unit-test existing methods
* Verify ChannelId, Color, UserId
* Wait - some slack urls may not correspond to default URL parsing rules? why?
* How do I deal with <@user> (i.e. slack control characters) vs. literals?

## Server
* Periodically post messages
* Respond to a slash command

## TODO
* Build into server
* Document existing code
* Build out additional APIs. Start with:
  * channels
  * chat
  * reminders
  * users
  * search

## Bot vs. App
* Can only access subset of API calls: https://api.slack.com/bot-users#bot-methods
* Bot tokens start with `xoxb`

## Levels
* Incoming webhooks (just send a message to a webhook)
* Slash command (process message from configured webhook and return a JSON message response)
* Bot users
  * Custom bot
  * App bot

## Slash Commands
* Slack posts to a URL you specify
* When you receive the message you have to respond on the same connection

## Events
* Can only be consumed as part of an oauth flow
* Does oauth have to be part of the library?
* Event:
  * metadata
  * payload -> T which derives from marker trait SlackEvent
* server-managed or lib-managed events?
