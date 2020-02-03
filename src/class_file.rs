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

struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<CpInfo>,
    access_flags: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

// struct StackMapTable {
//     attribute_name_index: u16,
//     attribute_length: u32,
//     entries: Vec<StackMapFrame>,
// }

// #[repr(C)]
// union StackMapFrame {
//     same_frame: SameFrame,
//     same_locals_1_stack_item_frame: SameLocals1StackItemFrame,
//     same_locals_1_stack_item_frame_extended: SameLocals1StackItemFrameExtended,
//     chop_frame: ChopFrame,
//     same_frame_extended: SameFrameExtended
// }
