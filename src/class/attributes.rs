use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub struct Attribute {
    attribute_name_index: u16,
    info: Vec<u8>,
}

impl Attribute {
    pub fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Attribute {

    }
}