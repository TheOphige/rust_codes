
# Calculator Microservice

This project is a simple calculator microservice built with Rust and Actix Web. It exposes HTTP endpoints for basic arithmetic operations:

- Addition: `/add/{a}/{b}`
- Subtraction: `/subtract/{a}/{b}`
- Multiplication: `/multiply/{a}/{b}`
- Division: `/divide/{a}/{b}`

Each endpoint takes two integer parameters and returns the result of the operation. The core logic is implemented in the library (`lib.rs`), and the web server routes are defined in `main.rs`.

To run the service, start the server and access the endpoints via your browser or API client.
