# ppsaver

This is an application that helps you force stop some heavy processes which make your computer freezing.

## Introduction

`ppsaver` is an application designed to help you force stop heavy processes which make your computer freeze.

In addition to this, the application includes a feature that allows it to watch for changes to the IP address and send notifications contains list of addresses (currently only supporting Telegram channels) when the IP address changes. Users can then access a dashboard by visiting one of the addresses in the list and close any heavy process by clicking on its PID number.

This application is written in Rust and can be easily installed by following the installation instructions provided below. The ppsaver application is open source and contributions are welcome. If you encounter any issues or have any suggestions for improvement, please submit a pull request or open an issue on the project's GitHub page.

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
