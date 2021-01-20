mod communication_utils;
use communication_utils::run_server_and_connect;
use spyware::communication::messages::DownloadFileRequest;
use spyware::communication::serialization::serialize_message;
use spyware::communication::server::get_message;
use std::io::Write;

#[test]
fn test_download_file_request() {
    println!("Running a server in the background");
    let mut stream = run_server_and_connect().expect("Error in running server :(");
    let message = DownloadFileRequest {
        path: String::from("/tmp/thefile"),
    };
    let buffer = serialize_message(
        spyware::communication::messages::Message::DownloadFileRequest { 0: message },
    )
    .unwrap();
    println!("Sending download file request to server");
    stream.write(&buffer).unwrap();
    println!("Trying to get response from server...");
    match get_message(&stream) {
        Ok(message) => println!("Wow, the message! {:?}", message),
        Err(_e) => println!("Shit, an error"),
    };
}
