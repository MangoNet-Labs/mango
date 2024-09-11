# Rust gRPC Project

## Project Overview

This project implements a simple gRPC service where the client can call a remote procedure on the server.

### What is gRPC?

gRPC is a serialization mechanism similar to JSON, but uses Protocol Buffers as its data format. It is efficient for handling structured data of several megabytes in size. gRPC provides a cross-platform Remote Procedure Call (RPC) mechanism.

### How to work?

RPC (Remote Procedure Call) allows the client to call methods on the server using a specific IP and port. The general workflow is as follows:
- **Service defined in proto**: The `service` in the proto file defines the RPC methods provided by the server. The client calls these methods to communicate with the server.
- **Client-server connection**: The client first establishes a connection, sends a request, calls the remote method, and the server responds after processing the request.

## Environment Setup

Ensure you have installed [Rust](https://www.rust-lang.org/) and [cargo](https://doc.rust-lang.org/cargo/).  
This project uses `tonic` as the gRPC framework.


## How to Run

Start the server and client:
```bash
cargo run --bin server
cargo run --bin client
```

## Output

```
Server Output：

Recorder listening on [::1]:50050
request: Request {
    metadata: MetadataMap {
        headers: {
            "te": "trailers",
            "content-type": "application/grpc",
            "user-agent": "tonic/0.7.2",
        },
    },
    message: RecordRequest {
        user_name: "Jeffy",
        user_age: 25,
    },
    extensions: Extensions,
}

```
```
Client Output：


Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.94s
     Running `target/debug/client`
Metadata response from server is: "User Jeffy is 25 old"

```
