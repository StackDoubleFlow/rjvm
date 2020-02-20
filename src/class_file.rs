struct CpInfo {
    tag: u8,
    info: Vec<u8>,
}

struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    desciptor_index: u16,
    attribues: Vec<AttributeInfo>,
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    desciptor_index: u16,
    attribues: Vec<AttributeInfo>,
}

struct AttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}

struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

struct CodeAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTable>,
    attributes: Vec<AttributeInfo>,
}

enum Constant {
    Class {
        name_index: u16
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16
    },
    InterfaceMethodref {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16
    },
    String {
        string_index: u16
    },
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16
    },
    MethodType{
        desciptor_index: u16
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16
    },
    Module,
    Package
}

impl Constant {

}

struct ClassAccessFlags {
    acc_public: bool,
    acc_final: bool,
    acc_super: bool,
    acc_interface: bool,
    acc_abstract: bool,
    acc_synthetic: bool,
    acc_annotation: bool,
    acc_enum: bool,
    acc_module: bool
}

struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant>,
    access_flags: ClassAccessFlags,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}
