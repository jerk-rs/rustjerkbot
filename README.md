# RUSTJERKBOT

[![Travis](https://img.shields.io/travis/rossnomann/rustjerkbot.svg?style=flat-square)](https://travis-ci.org/rossnomann/rustjerkbot)

## Environment variables

- `RUSTJERKBOT_TOKEN` - Telegram bot token
- `RUSTJERKBOT_PROXY` - Optional http(s) or socks(4/5) proxy url
                        (see https://docs.rs/tgbot/0.4.0/tgbot/struct.Config.html#method.proxy for more information)
- `RUSTJERKBOT_REDIS_URL` - Redis URL: redis://localhost for example
- `RUSTJERKBOT_CHAT_ID` - Chat ID to work with
- `RUSTJERKBOT_SHIPPERING_PAIR_TIMEOUT` - Timeout between pairs in seconds
- `RUSTJERKBOT_SHIPPERING_MESSAGE_TIMEOUT` - Timeout between shippering messages in seconds
- `RUSTJERKBOT_WEBHOOK_ADDRESS` - Server address for webhooks (`127.0.0.1` for example), optional.
- `RUSTJERKBOT_WEBHOOK_PATH` - Path for webhooks, must start with `/`. It's recommended to use a random string. Default values is: `/`.

If `RUSTJERKBOT_WEBHOOK_ADDRESS` is not specified, updates will be received using long-polling.

## LICENSE

The MIT License (MIT)
