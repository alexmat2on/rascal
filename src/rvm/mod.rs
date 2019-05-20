///
///
/// Instruction Set:
///     0x00 -> OP_EXIT   -  Terminate execution
///     0x01 -> OP_PUSH   -  Push 4 bytes onto stack
///     0x02 -> OP_POP    -  Pop x bytes from stack
///     0x03 -> OP_STORE  -  Store: Store value at [sp] into DATA address given by [sp - 1]
///     0x04 -> OP_LOAD   -  Load: Push value from DATA[stack[sp]] onto stack.
///     0x10 -> OP_ADD    -  Add two values from stack
///     0x11 -> OP_SUB    -  Subtract
///     0x20 -> OP_WRITE  -  Write the top element of stack to stdout
///     0x30 -> OP_JTRUE  -  Jump to address if top of stack is true.
///     0x31 -> OP_JFALSE -  Jump to address if top of stack is false.
///     0x32 -> OP_JMP    -  Jump to address.
///     0x40 -> OP_EQL    -  Determine if two top stack elements are boolean equal
///     0x41 -> OP_NEQL   -  Determine if two top stack elements are not boolean equals
///     0x42 -> OP_AND    -  Evaluate boolean AND with top two stack
///     0x43 -> OP_OR     -  Evaluate boolean OR with top two stack
///     0x44 -> OP_LT     -  Determine if top stack element is less than bottom.
///     0x45 -> OP_LTE    -  Determine if top stack element is less than or equal to bottom
///     0x46 -> OP_GT     -  Determine if top stack element is greater than bottom.
///     0x47 -> OP_GTE    -  Determine if top stack element is greater than or equal to bottom
///
use std::convert::TryInto;

pub struct RvmMachine {
    code: Vec<u8>,
    data: Vec<u8>,
    stack: RvmStack<u8>,
    ip: usize,
    sp: usize
}

impl RvmMachine {
    pub fn new(code: Vec<u8>) -> RvmMachine {
        RvmMachine {
            code,
            data: vec![0; 256],
            stack: RvmStack::new(0, 256),
            ip: 0,
            sp: 0
        }
    }

    pub fn exec(&mut self) {
        loop {
            let opcode = self.code[self.ip];
            match opcode {
                0x00 => break,
                0x01 => {
                    self.stack.push(self.code[self.ip + 1]);
                    self.stack.push(self.code[self.ip + 2]);
                    self.stack.push(self.code[self.ip + 3]);
                    self.stack.push(self.code[self.ip + 4]);
                    self.ip += 4;
                },
                0x02 => self.sp -= 1,
                0x03 => self.store(),
                0x04 => self.load(),
                0x10 => self.do_u32_binary(|a, b| a + b),
                0x11 => self.do_u32_binary(|a, b| b - a),
                0x12 => self.do_u32_binary(|a, b| a * b),
                0x13 => self.do_u32_binary(|a, b| b / a),
                0x20 => self.write_top(),
                0x30 => self.jmps(|v| v != 0, true),
                0x31 => self.jmps(|v| v == 0, true),
                0x32 => self.jmps(|v| true, false),
                0x40 => self.do_bool_binary(|a, b| a == b),
                0x41 => self.do_bool_binary(|a, b| a != b),
                0x42 => self.do_bool_binary(|a, b| {a != 0 && b != 0}),
                0x43 => self.do_bool_binary(|a, b| {a != 0 || b != 0}),
                // TO-DO: Are these comparison operations backwards?
                0x44 => self.do_bool_binary(|a, b| {a > b}),
                0x45 => self.do_bool_binary(|a, b| {a >= b}),
                0x46 => self.do_bool_binary(|a, b| {a < b}),
                0x47 => self.do_bool_binary(|a, b| {a <= b}),
                _ => {
                    panic!("Illegal RVM instruction. Dump...\n\n");
                    println!("{:?}", self.code);
                    self.stack.print(false);
                }
            }

            self.ip += 1;
        }

        self.stack.print(false);
    }

    fn write_top(&mut self) {
        let a = read_be_u32(&mut self.stack.pop(4));
        println!("{}", a);
    }

    fn do_u32_unary<F>(&mut self, unary_op: F) where
    F: Fn(u32) -> u32 {
        let a = read_be_u32(&mut self.stack.pop(4));
        let result = unary_op(a).to_be_bytes();

        self.stack.push(result[0]);
        self.stack.push(result[1]);
        self.stack.push(result[2]);
        self.stack.push(result[3]);
    }

    fn do_u32_binary<F>(&mut self, binary_op: F) where
    F: Fn(u32, u32) -> u32 {
        let a = read_be_u32(&mut self.stack.pop(4));
        let b = read_be_u32(&mut self.stack.pop(4));
        let result = binary_op(a, b).to_be_bytes();

        self.stack.push(result[0]);
        self.stack.push(result[1]);
        self.stack.push(result[2]);
        self.stack.push(result[3]);
    }

    fn do_bool_binary<F>(&mut self, binary_op: F) where
    F: Fn(u32, u32) -> bool {
        let a = read_be_u32(&mut self.stack.pop(4));
        let b = read_be_u32(&mut self.stack.pop(4));

        if binary_op(a, b) {
            self.stack.push(0);
            self.stack.push(0);
            self.stack.push(0);
            self.stack.push(1);
        } else {
            self.stack.push(0);
            self.stack.push(0);
            self.stack.push(0);
            self.stack.push(0);
        }
    }

    fn store(&mut self) {
        let value = read_be_u32(&mut self.stack.pop(4));
        let val_bytes = value.to_be_bytes();

        let address = read_be_u32(&mut self.stack.pop(4));

        self.data[address as usize] = val_bytes[0];
        self.data[(address + 1) as usize] = val_bytes[1];
        self.data[(address + 2) as usize] = val_bytes[2];
        self.data[(address + 3) as usize] = val_bytes[3];
    }

    fn load(&mut self) {
        let address = read_be_u32(&mut self.stack.pop(4));

        self.stack.push(self.data[address as usize]);
        self.stack.push(self.data[(address + 1) as usize]);
        self.stack.push(self.data[(address + 2) as usize]);
        self.stack.push(self.data[(address + 3) as usize]);
    }

    fn jmps<F>(&mut self, cond: F, check: bool) where
    F: Fn(u32) -> bool {
        let addr = read_be_u32(&mut self.stack.pop(4));
        let val;
        if check {
            val = read_be_u32(&mut self.stack.pop(4));
        } else {
            val = 0
        }
        if cond(val) {
            // Subtract a "1" because after outside match, ip is incremented.
            self.ip = addr as usize - 1;
        }
    }
}

#[derive(Clone)]
struct RvmStack<T> {
    stack: Vec<T>,
    pub sp: usize
}

impl<T: std::fmt::Debug + Clone + std::fmt::Display> RvmStack<T> {
    pub fn new(val: T, siz: usize) -> RvmStack<T> {
        RvmStack {
            stack: vec![val; siz],
            sp: 0
        }
    }

    pub fn print(&self, debug: bool) {
        let mut counter = 0;
        loop {
            if counter == self.sp {
                break;
            }

            println!("s[{}]: {}", counter, self.stack[counter + 1]);
            counter += 1;
        }

        if debug { println!("{:?}", self.stack)}
    }

    pub fn push(&mut self, byte: T) {
        self.sp += 1;
        self.stack[self.sp] = byte;
    }

    pub fn pop(&mut self, num: usize) -> &[T] {
        let start = self.sp - num + 1;
        self.sp -= num;

        &self.stack[start..]
    }
}

fn read_be_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}
