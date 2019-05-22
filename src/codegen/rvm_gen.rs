use super::CodeGenerator;

pub struct RvmGenerator {
    pub i_ptr: usize,
    pub data_addr: u32,
    pub code: Vec<u8>,
}

impl RvmGenerator {
    pub fn new() -> RvmGenerator {
        RvmGenerator { i_ptr : 0, data_addr: 0, code: vec![] }
    }
}

impl CodeGenerator for RvmGenerator {
    fn op(&mut self, opcode: &str) {
        self.i_ptr += 1;
        match opcode {
            "OP_EXIT" => self.code.push(0x00),
            "OP_PUSH" => self.code.push(0x01),
            "OP_POP" => self.code.push(0x02),
            "OP_STORE" => self.code.push(0x03),
            "OP_LOAD" => self.code.push(0x04),
            "OP_CALL" => self.code.push(0x05),
            "OP_RETURN" => self.code.push(0x06),
            "OP_ADD" => self.code.push(0x10),
            "OP_SUB" => self.code.push(0x11),
            "OP_MULT" => self.code.push(0x12),
            "OP_DIVI" => self.code.push(0x13),
            "OP_EQUAL" => self.code.push(0x40),
            "OP_NEQUAL" => self.code.push(0x41),
            "OP_AND" => self.code.push(0x42),
            "OP_OR" => self.code.push(0x43),
            "OP_LT" => self.code.push(0x44),
            "OP_LTE" => self.code.push(0x45),
            "OP_GT" => self.code.push(0x46),
            "OP_GTE" => self.code.push(0x47),
            "OP_WRITE" => self.code.push(0x20),
            "OP_JTRUE" => self.code.push(0x30),
            "OP_JFALSE" => self.code.push(0x31),
            "OP_JMP" => self.code.push(0x32),
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
        self.i_ptr += dsize;
    }

    fn fill(&mut self, data: String, dtype: &str, _dsize: usize) {
        let value_parsed;
        match dtype {
            "u32" => value_parsed = data.parse::<u32>().expect("Expected u32 conversion"),
            _ => panic!("Invalid data type specified.")
        };

        let value_bytes = value_parsed.to_be_bytes().to_vec();
        self.code[self.i_ptr] = value_bytes[0];
        self.code[self.i_ptr + 1] = value_bytes[1];
        self.code[self.i_ptr + 2] = value_bytes[2];
        self.code[self.i_ptr + 3] = value_bytes[3];
    }
}
