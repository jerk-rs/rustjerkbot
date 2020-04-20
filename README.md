# RUSTJERKBOT

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rossnomann/rustjerkbot/CI?style=flat-square)](https://github.com/rossnomann/rustjerkbot/actions/)

## Environment variables

- `RUSTJERKBOT_TOKEN` - Telegram bot token
- `RUSTJERKBOT_PROXY` - Optional http(s) or socks(4/5) proxy url
                        (see https://docs.rs/tgbot/0.4.0/tgbot/struct.Config.html#method.proxy for more information)
- `RUSTJERKBOT_REDIS_URL` - Redis URL: localhost:6379 for example
- `RUSTJERKBOT_POSTGRES_URL` - PostgreSQL URL: postgresql://user:pass@host/database
- `RUSTJERKBOT_CHAT_ID` - Chat ID to work with.
- `RUSTJERKBOT_WEBHOOK_ADDRESS` - Server address for webhooks (`127.0.0.1` for example), optional.
- `RUSTJERKBOT_WEBHOOK_PATH` - Path for webhooks, must start with `/`. It's recommended to use a random string. Default values is: `/`.

If `RUSTJERKBOT_WEBHOOK_ADDRESS` is not specified, updates will be received using long-polling.

## LICENSE

The MIT License (MIT)
