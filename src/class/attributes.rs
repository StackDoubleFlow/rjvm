use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

pub struct Attribute {
    attribute_name_index: u16,
    info: Vec<u8>,
}

impl Attribute {
    pub fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Attribute {
        Attribute {
            attribute_name_index: 0,
            info: Vec::new(),
        }
    }
}
