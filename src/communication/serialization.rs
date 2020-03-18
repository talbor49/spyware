use std::io::{Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;

use crate::communication::messages::{MESSAGE_HEADER_LENGTH, MESSAGE_TYPE_SIZE, MessageType};

pub fn extract_msg_type_and_length(type_and_length: [u8; MESSAGE_HEADER_LENGTH]) -> (u8, usize) {
    let msg_type = type_and_length[0];
    let msg_length = &type_and_length[MESSAGE_TYPE_SIZE..MESSAGE_HEADER_LENGTH];
    let mut rdr = Cursor::new(msg_length);
    let msg_length = rdr.read_u32::<BigEndian>().unwrap() as usize;
    return (msg_type, msg_length);
}

pub fn serialize_message(message: impl Serialize + MessageType) -> Result<Vec<u8>, Error> {
    let serialized_message = ron::ser::to_string(&message).unwrap_or_else(|e| {
        // TODO handle this better
        panic!("Serialization error, error: {}", e.to_string());
    });
    let message_len = serialized_message.len();

    let mut buffer: Vec<u8> = Vec::with_capacity(message_len + MESSAGE_HEADER_LENGTH);
    buffer.push(message.get_type());
    buffer.write_u32::<BigEndian>(message_len as u32)?;
    buffer.extend(serialized_message.into_bytes());
    Ok(buffer)
}
