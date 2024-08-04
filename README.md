# TechNews

A Rust-based application to fetch and display the latest news from various sources.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Installation](#installation)
- [Usage](#usage)

## Introduction

The TechNews application fetches and displays the latest news from multiple sources. It is written in Rust and aims to provide a simple, efficient, and reliable way to stay updated with current events.

## Features

- Fetches news from multiple sources.
- Displays news in a user-friendly format.

## Getting Started
### Prerequisites

- Rust (latest stable version)
- An SMTP server for sending emails
- Environment variables for SMTP configuration

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/cadusouza1/technews.git
    cd technews
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
