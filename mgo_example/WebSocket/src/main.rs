use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use futures::StreamExt;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter; 
use log::info;

static NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

/*
* The Web server will serve static files in the static directory locally on port 7070
* The information passing between different clients (WebSockets) via tokio channel is stored in the V of the Map
*/
#[tokio::main]
async fn main() {

    //Set the "RUST APP LOG" environment variable to "debug"
    env::set_var("RUST_APP_LOG", "debug");
    //Initialize the logger, using the logging level specified by the "RUST_APP_LOG" environment variable
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let users = Users::default();
    //Annotate it with a websocket request instead of an http request and expect to pass it to users
    let chat = warp::path("ws")
        .and(warp::ws())
        .and(with_user(users))
        .map(|ws:warp::ws::Ws, users| ws.on_upgrade(move |socket| conn(socket, users))); /*获取某些东西*/
    
    //Create a Warp filter that points to the "static" directory
    let files = warp::fs::dir("static");
    //Set up add route
    let router = chat.or(files);
    //Use Warp to serve static files, listening on port 7070 at the local address 127.0.0.1
    warp::serve(router).run(([127, 0, 0, 1], 7070)).await;

    println!("Hello world");
}


//Upgrade when a users action is handled asynchronously
fn with_user(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    //Move users into it by cloning
    warp::any().map(move || users.clone())
}

//Connect and send messages asynchronously
async fn conn(ws: WebSocket, users: Users) {
    //Get the id to set up the connection
    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    info!("Connected UserId by {}", my_id);
    //Separating flow information
    let (user_tx, mut user_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::spawn(rx.forward(user_tx));
    //Add users
    users.write().await.insert(my_id, tx);

    //Read and broadcast send messages
    while let Some(result) = user_rx.next().await {
        broadcast(result.unwrap(), &users).await;
    }
    disconn(my_id, &users).await;
}

//Get the msg broadcast message send for the users map
async fn broadcast(msg: Message, users: &Users) {
    if let Ok(_) = msg.to_str() {
        for (&uid, tx) in users.read().await.iter() {
            info!("uid is {} and message is {:?}", uid, msg.clone());
            tx.send(Ok(msg.clone())).expect("Failed to send message!");
        }
    }
}

//disconnected
async fn disconn(my_id: usize, users: &Users) {
    info!("GoodBye {}", my_id);
    users.write().await.remove(&my_id);
}