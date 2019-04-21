use super::CodeGenerator;

pub struct RvmGenerator {
    i_ptr: usize,
    pub code: Vec<u8>,
}

impl RvmGenerator {
    pub fn new() -> RvmGenerator {
        RvmGenerator { i_ptr : 0, code: vec![] }
    }
}

impl CodeGenerator for RvmGenerator {
    fn op(&mut self, opcode: &str) {
        match opcode {
            "OP_EXIT" => self.code.push(0x00),
            "OP_PUSH" => self.code.push(0x01),
            "OP_POP" => self.code.push(0x02),
            "OP_ADD" => self.code.push(0x10),
            _ => panic!("Invalid op code given.")
        }
    }

    fn data(&mut self, data: String, dtype: &str, dsize: usize) {
        let value_parsed;
        match dtype {
            "u32" => value_parsed = data.parse::<u32>().expect("Expected u32 conversion"),
            _ => panic!("Invalid data type specified.")
        };

        let mut value_bytes = value_parsed.to_be_bytes().to_vec();
        self.code.append(&mut value_bytes);
    }
}