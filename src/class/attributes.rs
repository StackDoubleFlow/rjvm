use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

enum VerificationType {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object {
        cpool_index: u16
    },
    Uninitialized {
        offset: u16
    }
}

enum StackMapFrame {
    Same,
    SameLocalsOneStackItem,
    SameLocalsOneStackItemExtended,
    Chop,
    SameExtended,
    Append,
    Full
}

#[derive(Debug)]
pub struct AttributeInfo {
    name_index: u16,
    data: Vec<u8>,
}

impl AttributeInfo {
    pub fn deserialize(reader: &mut Cursor<Vec<u8>>) -> AttributeInfo {
        let name_index = reader.read_u16::<BigEndian>().unwrap();
        let length = reader.read_u32::<BigEndian>().unwrap();
        let mut data = vec![0u8; length as usize];
        reader.read_exact(data.as_mut_slice()).unwrap();
        AttributeInfo { name_index, data }
    }
}

#[derive(Debug)]
struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Exception {
    fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Exception {
        Exception {
            start_pc: reader.read_u16::<BigEndian>().unwrap(),
            end_pc: reader.read_u16::<BigEndian>().unwrap(),
            handler_pc: reader.read_u16::<BigEndian>().unwrap(),
            catch_type: reader.read_u16::<BigEndian>().unwrap(),
            
        }
    }
}

trait Attribute {
    fn deserialize(data: Vec<u8>) -> Self;
}

struct ConstantValueAttribute {
    index: u16
}

impl Attribute for ConstantValueAttribute {
    fn deserialize(data: Vec<u8>) -> Self {
        ConstantValueAttribute {
            index: Cursor::new(data).read_u16::<BigEndian>().unwrap()
        }
    }
}

struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<Exception>,
    attributes: Vec<AttributeInfo>
}

impl Attribute for CodeAttribute {
    fn deserialize(data: Vec<u8>) -> Self {
        let mut reader = Cursor::new(data);

        let max_stack = reader.read_u16::<BigEndian>().unwrap();
        let max_locals = reader.read_u16::<BigEndian>().unwrap();

        let code_length = reader.read_u16::<BigEndian>().unwrap();
        let mut code = vec![0u8; code_length as usize];
        reader.read_exact(code.as_mut_slice()).unwrap();

        let exception_table_length = reader.read_u16::<BigEndian>().unwrap();
        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
        for _ in 0..exception_table_length {
            exception_table.push(Exception::deserialize(&mut reader));
        }

        let attributes_count = reader.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::deserialize(&mut reader));
        }

        CodeAttribute {
            max_stack, max_locals,
            code, exception_table,
            attributes
        }
    }
}