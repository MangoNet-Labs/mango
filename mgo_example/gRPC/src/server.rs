use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService{}

#[tonic::async_trait]

/* 
* service implementation
* func: Implements the Recorder service interface defined in the proto file
* service: This implements the send_message method defined in the proto file
* param: Receives a message from the client and returns a RecordResponse containing the message
*/
impl Recorder for RecorderService {
    async fn send_message(
        &self,
        /*gRPC request object */
        request: Request<RecordRequest>
    ) -> Result<Response<RecordResponse>, Status> {
        println!("request: {:#?}", request);
        //into_inner gets the request message entity which is the RecordRequest
        let req = request.into_inner();
        let response = RecordResponse {
            successful: true,
            message: format!("User {} is {} old", req.user_name, req.user_age).into()
        };
        Ok(Response::new(response))
    }
}

/*Start server*/
#[tokio::main]
// Take the request and return a string to the client
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    let recorder = RecorderService::default();
    println!("Recorder listening on {}", addr);

    // Create a gRPC server
    Server::builder()
        // Add the service we defined to the server
        .add_service(RecorderServer::new(recorder))
        // Specify the address and port the server listens on and start the server
        .serve(addr)
        .await?;

    Ok(())
}