use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::communication::messages::{MESSAGE_HEADER_LENGTH, MESSAGE_TYPE_SIZE};

pub fn extract_msg_type_and_length(type_and_length: [u8; MESSAGE_HEADER_LENGTH]) -> (u8, usize) {
    let msg_type = type_and_length[0];
    let msg_length = &type_and_length[MESSAGE_TYPE_SIZE..MESSAGE_HEADER_LENGTH];
    let mut rdr = Cursor::new(msg_length);
    let msg_length = rdr.read_u32::<BigEndian>().unwrap() as usize;
    return (msg_type, msg_length);
}
