use super::Mem;
use crate::common::abi::*;
use crate::common::build::*;
#[derive(Debug)]
pub struct MemWriter {
    pub write_en: PortRef,
    write_en_cache: u32,
    pub addr: PortRef,
    addr_cache: u32,
    pub write_data: PortRef,
    write_data_cache: u32,
    mem: IndexPortShared<Mem>,
}
impl MemWriter {
    pub fn new(mem: IndexPortShared<Mem>) -> Self {
        Self {
            write_en: bomb().into(),
            write_en_cache: 0,
            addr: bomb().into(),
            addr_cache: 0,
            write_data: bomb().into(),
            write_data_cache: 0,
            mem,
        }
    }
}
impl Control for MemWriter {
    fn rasing_edge(&mut self) {
        if self.write_en.read() == 1 {
            self.write_en_cache = 1;
            self.addr_cache = self.addr.read();
            self.write_data_cache = self.write_data.read();
        } else {
            self.write_en_cache = 0;
        }
    }
    fn falling_edge(&mut self) {
        if self.write_en_cache == 1 {
            self.mem
                .borrow_mut()
                .write(self.addr_cache as usize, self.write_data_cache);
        }
    }
}
