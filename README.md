# Axum Web Application

This is a simple web application built using the Axum framework in Rust. The application demonstrates how to set up a basic web server, handle requests, and define routes.

## Project Structure

```
axum-web-app
├── src
│   ├── main.rs          # Entry point of the application
│   ├── handlers         # Contains request handlers
│   │   └── mod.rs
│   ├── routes           # Defines application routes
│   │   └── mod.rs
│   └── models           # Data models used in the application
│       └── mod.rs
├── Cargo.toml           # Cargo configuration file
└── README.md            # Project documentation
```

## Setup Instructions

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rustup.rs](https://rustup.rs/).

2. Clone the repository:

   ```
   git clone <repository-url>
   cd axum-web-app
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the application:

   ```
   cargo run
   ```

## Usage

Once the application is running, you can access it at `http://localhost:3000`. You can test the endpoints using tools like `curl` or Postman.

### Example Endpoints

- **GET /users**: Retrieve a list of users.
- **POST /users**: Create a new user.

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the project.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Mac

```shell
cargo run
error: package `mio v1.0.3` cannot be built because it requires rustc 1.70 or newer, while the currently active rustc version is 1.65.0
Either upgrade to rustc 1.70 or newer, or use
cargo update -p mio@1.0.3 --precise ver
where `ver` is the latest version of `mio` supporting rustc 1.65.0
```

