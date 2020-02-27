use attributes::AttributeInfo;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};
use std::str;

mod attributes;

bitflags! {
    struct FieldAccessFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_PRIVATE = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC = 0x0008;
        const ACC_FINAL = 0x0010;
        const ACC_VOLATILE = 0x0040;
        const ACC_TRANSIENT = 0x0080;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ENUM = 0x4000;
    }
}

#[derive(Debug)]
struct Field {
    access_flags: FieldAccessFlags,
    name_index: u16,
    desciptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl Field {
    fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Field {
        let access_flags =
            FieldAccessFlags::from_bits_truncate(reader.read_u16::<BigEndian>().unwrap());
        let name_index = reader.read_u16::<BigEndian>().unwrap();
        let desciptor_index = reader.read_u16::<BigEndian>().unwrap();

        let attributes_count = reader.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::deserialize(reader));
        }

        Field {
            access_flags,
            name_index,
            desciptor_index,
            attributes,
        }
    }
}

bitflags! {
    struct MethodAccessFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_PRIVATE = 0x0002;
        const ACC_PROTECTED = 0x0004;
        const ACC_STATIC = 0x0008;
        const ACC_FINAL = 0x0010;
        const ACC_SYNCHRONIZED = 0x0020;
        const ACC_BRIDGE = 0x0040;
        const ACC_VARARGS = 0x0080;
        const ACC_NATIVE = 0x0100;
        const ACC_ABSTRACT = 0x0400;
        const ACC_STRICT = 0x0800;
        const ACC_SYNTHETIC = 0x1000;
    }
}

#[derive(Debug)]
struct Method {
    access_flags: MethodAccessFlags,
    name_index: u16,
    desciptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl Method {
    fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Method {
        let access_flags =
            MethodAccessFlags::from_bits_truncate(reader.read_u16::<BigEndian>().unwrap());
        let name_index = reader.read_u16::<BigEndian>().unwrap();
        let desciptor_index = reader.read_u16::<BigEndian>().unwrap();

        let attributes_count = reader.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::deserialize(reader));
        }

        Method {
            access_flags,
            name_index,
            desciptor_index,
            attributes,
        }
    }
}

struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

impl Constant {
    fn deserialize(reader: &mut Cursor<Vec<u8>>) -> Constant {
        use Constant::*;
        let tag = reader.read_u8().unwrap();
        match tag {
            1 => {
                let length = reader.read_u16::<BigEndian>().unwrap();
                let mut buf = vec![0u8; length as usize];
                reader.read_exact(buf.as_mut_slice()).unwrap();
                Utf8(str::from_utf8(buf.as_slice()).unwrap().to_string())
            }
            3 => Integer(reader.read_u32::<BigEndian>().unwrap()),
            4 => Float(reader.read_f32::<BigEndian>().unwrap()),
            5 => Long(reader.read_u64::<BigEndian>().unwrap()),
            6 => Double(reader.read_f64::<BigEndian>().unwrap()),
            7 => Class {
                name_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            8 => String {
                string_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            9 => Fieldref {
                class_index: reader.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            10 => Methodref {
                class_index: reader.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            12 => InterfaceMethodref {
                class_index: reader.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            15 => MethodHandle {
                reference_kind: reader.read_u8().unwrap(),
                reference_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            16 => MethodType {
                descriptor_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            17 => Dynamic {
                bootstrap_method_attr_index: reader.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            18 => InvokeDynamic {
                bootstrap_method_attr_index: reader.read_u16::<BigEndian>().unwrap(),
                name_and_type_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            19 => Module {
                name_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            20 => Package {
                name_index: reader.read_u16::<BigEndian>().unwrap(),
            },
            _ => {
                panic!("Invalid constant pool tag");
            }
        }
    }
}

bitflags! {
    struct ClassAccessFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_FINAL = 0x0010;
        const ACC_SUPER = 0x0020;
        const ACC_INTERFACE = 0x0200;
        const ACC_ABSTRACT = 0x0400;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ANNOTATION = 0x2000;
        const ACC_ENUM = 0x4000;
        const ACC_MODULE = 0x8000;
    }
}

#[derive(Debug)]
pub struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant>,
    access_flags: ClassAccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<AttributeInfo>,
}

impl Class {
    pub fn deserialize(vec: Vec<u8>) -> Class {
        // TODO: Handle ClassFormatError

        let mut reader = Cursor::new(vec);
        let magic = reader.read_u32::<BigEndian>().unwrap();
        assert!(magic == 0xCAFEBABE);

        // TODO: Handle UnsupportedClassVersionError
        let minor_version = reader.read_u16::<BigEndian>().unwrap();
        let major_version = reader.read_u16::<BigEndian>().unwrap();

        let constant_pool_count = reader.read_u16::<BigEndian>().unwrap();
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize - 1);
        for _ in 0..(constant_pool_count - 1) {
            constant_pool.push(Constant::deserialize(&mut reader));
        }

        let access_flags =
            ClassAccessFlags::from_bits_truncate(reader.read_u16::<BigEndian>().unwrap());
        let this_class = reader.read_u16::<BigEndian>().unwrap();
        let super_class = reader.read_u16::<BigEndian>().unwrap();

        let interface_count = reader.read_u16::<BigEndian>().unwrap();
        let mut interfaces = Vec::with_capacity(interface_count as usize);
        for _ in 0..interface_count {
            interfaces.push(reader.read_u16::<BigEndian>().unwrap());
        }

        let fields_count = reader.read_u16::<BigEndian>().unwrap();
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Field::deserialize(&mut reader));
        }

        let methods_count = reader.read_u16::<BigEndian>().unwrap();
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Method::deserialize(&mut reader));
        }

        let attributes_count = reader.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::deserialize(&mut reader));
        }

        Class {
            major_version,
            minor_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    pub fn verify(&self) {
        // TODO: Verify class
    }
}

struct ClassLoadingConstraints {}

enum ClassLoaderRecordType {
    Initiating,
    Defining,
}

struct ClassLoaderRecordEntry {
    r#type: ClassLoaderRecordType,
    name: String,
}

struct BootstrapClassLoader {
    record: Vec<ClassLoaderRecordEntry>,
}

impl BootstrapClassLoader {
    fn get_record_exists(&self, name: &String) -> bool {
        self.record.iter().any(|r| r.name == *name)
    }

    pub fn load_class(&mut self, name: String) {
        if self.get_record_exists(&name) { panic!("LinkageError") }

        self.record.push(ClassLoaderRecordEntry {
            r#type: ClassLoaderRecordType::Defining,
            name
        });
    }

    pub fn load_class_with_constraints(&mut self, name: String, constraints: ClassLoadingConstraints) {
        if self.get_record_exists(&name) { panic!("LinkageError") }

        self.record.push(ClassLoaderRecordEntry {
            r#type: ClassLoaderRecordType::Defining,
            name
        });
    }
}
