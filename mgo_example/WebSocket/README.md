# Rust WebSocket

## Project Brief
The WebSocket project is a Warp-based and Tokio WebSocket service that can handle WebSocket connections from multiple users. Through the asynchronous messaging and broadcasting mechanism, the server can broadcast messages sent by one user to all connected users.
### 1、Project Introduction Use a Map to store different users
```
* Key: id(usize)
* Value: Channels to send to different users of the frontend
```

### 2. Core Function

The following functions are achieved through the filter system provided by Warp and the asynchronous processing capabilities of Tokio:
* Use the 'websocket' and 'on_upgrade' methods to handle WebSocket upgrade requests.
* The 'split' method is used to split the WebSocket stream into receiving and sending parts.
* Using Tokio's 'mpsc' channel to broadcast messages ensures that they are received by all connected users.

### 3、Message Broadcast Mechanism
When the server receives a WebSocket message from one user, it broadcasts the message to all users by iterating over all user send channels in the 'HashMap'.

---
## Dependency

The following main dependencies are used in this project:

- `warp`: Web framework with support for WebSocket and file serving.
- `tokio`: When running asynchronously, it supports the scheduling and asynchronous processing of concurrent tasks.
- `futures`: Provides asynchronous stream processing.
- `log`: Logger for debugging and tracing information
- `pretty_env_logger`: Simple log output configuration tool.

---
## Running
### Clone Project
```cookie
$ git clone <repository-url>
$ cd <project-directory>
```
### Build
```cargo build```
### Execute
Start the WebSocket server listening locally on port 127.0.0.1:7070:


```cargo run```
### Connected
You can connect to a WebSocket server through a WebSocket client (such as a browser or a dedicated utility) at the following URL:


```127.0.0.1:7070```
### Example log output
When the project runs, connected users and broadcast messages will be logged. Here is an example of the log output.I started three browser Windows to send and forward messages, as follows

```
 INFO  warp::server > Server::run; addr=127.0.0.1:7070
 INFO  warp::server > listening on http://127.0.0.1:7070
 DEBUG hyper::proto::h1::io > parsed 15 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG warp::filters::fs      > dir: appending index.html to directory path
 DEBUG hyper::proto::h1::io   > flushed 171 bytes
 DEBUG hyper::proto::h1::io   > flushed 1142 bytes
 DEBUG hyper::proto::h1::io   > parsed 14 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 177 bytes
 DEBUG hyper::proto::h1::io   > flushed 1638 bytes
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 1
 DEBUG hyper::proto::h1::io   > parsed 14 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG warp::filters::fs      > file not found: "static/favicon.ico"
 DEBUG warp::filter::service  > rejected: Rejection(NotFound)
 DEBUG hyper::proto::h1::io   > flushed 82 bytes
 INFO  ws_code                > uid is 1 and message is Text("11111")
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 2
 INFO  ws_code                > uid is 1 and message is Text("222")
 INFO  ws_code                > uid is 2 and message is Text("222")
 DEBUG hyper::proto::h1::io   > parsed 12 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 DEBUG hyper::proto::h1::io   > flushed 166 bytes
 INFO  ws_code                > Connected UserId by 3
 INFO  ws_code                > uid is 1 and message is Text("333")
 INFO  ws_code                > uid is 3 and message is Text("333")
 INFO  ws_code                > uid is 2 and message is Text("333")
 DEBUG hyper::proto::h1::conn > read eof
```