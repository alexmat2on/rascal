///
///
/// Instruction Set:
///     0x00 -> Terminate execution
///     0x01 -> Push 4 bytes onto stack
///     0x02 -> Pop value from stack
///     0x10 -> Add two values from stack
///
pub struct RvmMachine {
    code: Vec<u8>,
    stack: RvmStack<u32>,
    ip: usize,
    sp: usize
}

impl RvmMachine {
    pub fn new(code: Vec<u8>) -> RvmMachine {
        RvmMachine {
            code: code,
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
                    let int_bytes = [
                        self.code[self.ip + 1],
                        self.code[self.ip + 2],
                        self.code[self.ip + 3],
                        self.code[self.ip + 4]
                    ];
                    let parsed_int = u32::from_be_bytes(int_bytes);
                    self.stack.push(parsed_int);
                    self.ip += 5;
                },
                0x02 => self.sp -= 1,
                0x10 => {
                    self.add();
                    self.ip += 1;
                },
                0x11 => {
                    self.sub();
                    self.ip += 1;
                },
                _ => {
                    println!("Uh oh... dumping...");
                    println!("{:?}", self.code);
                    self.stack.print(false);
                    panic!("Illegal RVM instruction.");
                }
            }
        }

        self.stack.print(false);
    }

    fn add(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.pop();

        self.stack.push(a + b);
    }

    fn sub(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.pop();

        self.stack.push(b - a);
    }
}

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

            println!("{}", self.stack[counter + 1]);
            counter += 1;
        }

        if debug { println!("{:?}", self.stack)}
    }

    pub fn push(&mut self, data: T) {
        self.sp += 1;
        self.stack[self.sp] = data;
    }

    pub fn pop(&mut self) -> T {
        self.sp -= 1;
        self.stack[self.sp + 1].clone()
    }
}
