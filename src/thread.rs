use crate::class::Constant;

struct Thread {
    pc: usize,
}

struct Frame<'a> {
    // TODO: Local variables
    operand_stack: Vec<u8>,
    runtime_constant_pool: &'a Vec<Constant>,
}
