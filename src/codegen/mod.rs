pub mod rvm_gen;

pub trait CodeGenerator {
    fn op(&mut self, opcode: &str);
    fn data(&mut self, data: String, dtype: &str, dsize: usize);
}
