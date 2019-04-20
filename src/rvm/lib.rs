///
///
/// Instruction Set:
///     0x00 -> Terminate execution
///     0x01 -> Push 4 bytes onto stack
struct RvmMachine {
    code: Vec<u8>,
    stack: Vec<u32>,
    ip: usize,
    sp: usize
}

impl RvmMachine {
    pub fn new(code: Vec<u8>) -> {
        RvmMachine {
            code: code,
            stack: vec![]
        }
    }

    pub fn exec(&self) {
        loop {

        }
        for opcode in code {
            match opcode {
                0x00 =>
            }
        }
    }
}
