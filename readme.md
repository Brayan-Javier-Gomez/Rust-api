# RUST API

![Rust](https://img.shields.io/badge/Rust-🦀-orange)

Project of implementation of api rest in the RUST language, using the web frameworks tokio(asynchronous operations), actix web(web server), sqlx(ORM for postresql), openssl and RSA for encryption and digital signature, the objective of this project will be the generation of the basic structure of a discussion forum, exploring the security and speed capabilities of the rust language to get the most out of them.

## Table of Contents

- [Installation](#installation)

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) Installed on your system.
- [Cargo](https://doc.rust-lang.org/cargo/) to manage dependencies and tasks.
- [Openssl](https://github.com/openssl/openssl) (In case of windows, in linux systems it is installed by default.)
### Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/Brayan-Javier-Gomez/Rust-api.git
   cd Rust-api
2. Configure env file
    ```bash
    DATABASE_URL=postgres://user:password@localhost:5432/database_name
    SERVER_IP=127.0.0.1
    SERVER_PORT=8080
3. Build project and download dependencies
    ```bash
    cargo build
4. Run project in development mode
    ```bash
    cargo run
