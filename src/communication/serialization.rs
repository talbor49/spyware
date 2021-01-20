use std::io::{Cursor, Error};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::communication::messages::{Message, MESSAGE_HEADER_LENGTH};

pub fn extract_msg_type_and_length(type_and_length: [u8; MESSAGE_HEADER_LENGTH]) -> usize {
    let msg_length_off = &type_and_length[0..MESSAGE_HEADER_LENGTH];
    let mut rdr = Cursor::new(msg_length_off);
    rdr.read_u32::<BigEndian>().unwrap() as usize
}

pub fn serialize_message(message: Message) -> Result<Vec<u8>, Error> {
    let serialized_message = ron::ser::to_string(&message).unwrap_or_else(|e| {
        // TODO handle this better
        panic!("Serialization error, error: {}", e.to_string());
    });
    let message_len = serialized_message.len();

    let mut buffer: Vec<u8> = Vec::with_capacity(message_len + MESSAGE_HEADER_LENGTH);
    buffer.write_u32::<BigEndian>(message_len as u32)?;
    buffer.extend(serialized_message.into_bytes());
    Ok(buffer)
}
