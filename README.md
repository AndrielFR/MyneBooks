<!--
  ~ Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>
  ~ 
  ~ SPDX-License-Identifier: MIT
  -->

# MyneBooks - Telegram Bot

![License](https://img.shields.io/github/license/AndrielFR/MyneBooks)

MyneBooks is a bot to read or download books without leaving Telegram.

Developed in Rust using the MTProto library [grammers](https://github.com/Lonami/grammers).

## Preparing and running

Rename or copy `config.toml.sample` to `config.toml` and fill with your data like: <br>
```toml
[grammers]
api_id = 1234567
api_hash = "1z02nserl588a2tek491t74839941e29"
bot_token = "1234567890:A8BCD3Ef7ghijk1LmNO9pQr5stuvwX2Yz0A"

[myne]
prefixes = ["!", "/", ";"]
```

Run the bot with: <br>
```bash
cargo run --release
```

## License

Copyright © 2022 [AndrielFR](https://github.com/AndrielFR)

Licensed under the [Expat/MIT license](LICENSE).
This project is also [REUSE compliant](https://reuse.software/).
See individual files for more copyright information.
