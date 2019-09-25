use crate::communication::messages::{MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

pub fn get_msg_type_and_length(
    type_and_length: [u8; MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE],
) -> (u8, usize) {
    let msg_type = type_and_length[0];
    let msg_length = &type_and_length[MESSAGE_TYPE_SIZE..MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE];
    let mut rdr = Cursor::new(msg_length);
    let msg_length = rdr.read_u32::<BigEndian>().unwrap() as usize;
    return (msg_type, msg_length);
}
