///
///
/// Instruction Set:
///     0x00 -> Terminate execution
///     0x01 -> Push 4 bytes onto stack
///     0x02 -> Pop x bytes from stack
///     0x03 -> Store: Store value at [sp] into DATA address given by [sp - 1]
///     0x04 -> Load: Push value from DATA[stack[sp]] onto stack.
///     0x10 -> Add two values from stack
///     0x11 -> Subtract
///     0x20 -> Write the top element of stack to stdout
///
///
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
            code: code,
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
                0x10 => self.do_binary(|a, b| a + b),
                0x11 => self.do_binary(|a, b| b - a),
                0x12 => self.do_binary(|a, b| a * b),
                0x13 => self.do_binary(|a, b| b / a),
                0x20 => self.write_top(),
                _ => {
                    println!("Uh oh... dumping...");
                    println!("{:?}", self.code);
                    self.stack.print(false);
                    panic!("Illegal RVM instruction.");
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

    fn do_unary<F>(&mut self, unary_op: F) where
    F: Fn(u32) -> u32 {
        let a = read_be_u32(&mut self.stack.pop(4));
        let result = unary_op(a).to_be_bytes();

        self.stack.push(result[0]);
        self.stack.push(result[1]);
        self.stack.push(result[2]);
        self.stack.push(result[3]);
    }

    fn do_binary<F>(&mut self, binary_op: F) where
    F: Fn(u32, u32) -> u32 {
        let a = read_be_u32(&mut self.stack.pop(4));
        let b = read_be_u32(&mut self.stack.pop(4));
        let result = binary_op(a, b).to_be_bytes();

        self.stack.push(result[0]);
        self.stack.push(result[1]);
        self.stack.push(result[2]);
        self.stack.push(result[3]);
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
