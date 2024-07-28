# TechNews

TechNews is a Rust-based project designed to scrape news articles from tech websites and send a summary of the latest news via email. The initial implementation fetches news from Hackaday and sends an email with articles from the past 24 hours.

## Getting Started
### Prerequisites

- Rust (latest stable version)
- An SMTP server for sending emails
- Environment variables for SMTP configuration

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/cadusouza1/technews.git
    cd TechNews
    ```

2. Set up the environment variables required for SMTP configuration:
    ```sh
    export SMTP_USERNAME=your_smtp_username
    export SMTP_PASSWORD=your_smtp_password
    export SMTP_SERVER=your_smtp_server
    ```

3. Build the project:
    ```sh
    cargo build
    ```

### Usage

To run the project, execute the following command:
```sh
cargo run
```
