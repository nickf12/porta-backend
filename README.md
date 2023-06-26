# Porta BackEnd #


## Following Crates are used to build this project: ##
    
* axum – A Rust web application framework built on top of Tokio,   featuring a flexible routing system, middleware, and support for JSON and form-encoded request bodies.
* tokio – A Rust runtime for building reliable and asynchronous I/O services, such as network connections, filesystem access, and timers.
* chrono – A Rust library for working with date and time.
* serde – A Rust library for serializing and deserializing data structures to and from JSON, YAML, and other formats.
* serde_json – A Rust library that provides JSON serialization and deserialization based on Serde.
* uuid – A Rust library for generating, parsing, and manipulating UUIDs.
* tower-http -A Rust library that provides HTTP middleware and utilities for use with the Tower framework.


### to build the project and start the server. This command watches the ‘src‘ directory for changes and restarts the server accordingly: ###

``` cargo watch -q -c -w src/ -x run ```

Navigate to  http://localhost:8000/api/porta
