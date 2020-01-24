mod communication_utils;
use communication_utils::run_server_and_connect;
use rustdoor::communication::messages::DownloadFileRequest;
use rustdoor::communication::serialization::serialize_message;
use std::io::{Read, Write};

#[test]
fn test_download_file_request() {
    let mut stream = run_server_and_connect().unwrap();
    let message = DownloadFileRequest {
        path: String::from("/tmp/thefile"),
        async_run: false,
    };
    let buffer = serialize_message(message).unwrap();
    stream.write(&buffer).unwrap();
    let mut response_buffer = Vec::new();
    stream
        .read(&mut response_buffer)
        .expect("Could not read response from server");
    println!("The read file response: {:?}", &response_buffer);
}
