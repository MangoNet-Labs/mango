/* Client proto is generated automatically */
use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;
use log::{info, error};

pub mod records {
    tonic::include_proto!("records");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

     // Establish a connection to the server
     info!("Connecting to server...");
    let mut client  = RecorderClient::connect("http://[::1]:50050").await?;
    
    // Request
    info!("Creating request...");
    let request = Request::new(
        RecordRequest {
            user_name: "Jeffy".to_string(),
            user_age: 25,
        }
    );
    
    // Send a request and get a response
    info!("Sending request...");
    let response= client.send_message(request).await?;
 
    if response.get_ref().successful {
        info!("Metadata response from server is: {:#?}", response.get_ref().message);
        println!("Metadata response from server is: {:#?}", response.get_ref().message);
    } else {
        error!("Request failed.");
    }

    Ok(())
}