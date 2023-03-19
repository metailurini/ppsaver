# ppsaver

This is an application that helps you force stop some heavy processes which make your computer freezing.

## Introduction

This application is designed to help you force stop some heavy processes which make your computer freezing.

## Installation

1. Clone the repository.
2. Run `cargo build --release` to build the binary.
3. Copy `./target/release/ppsaver` to `/usr/bin`.

## Usage

- Run `/usr/bin/ppsaver`.
- Set the following environment variables:
    - `IP`: IP address to bind to.
    - `PORT`: Port to bind to.
    - `TELEGRAM_URL`: Telegram API URL.
    - `TELEGRAM_CHAT_ID`: Telegram chat ID.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Push your changes to your fork.
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
