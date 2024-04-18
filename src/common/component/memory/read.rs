use crate::common::abi::*;
use crate::common::build::*;
#[derive(Debug, Clone)]
pub struct MemReader {
    pub read_en: PortRef,
    pub addr: PortRef,
    mem: IndexPortRef,
}
impl MemReader {
    pub fn new(mem: IndexPortRef) -> Self {
        Self {
            read_en: bomb().into(),
            addr: bomb().into(),
            mem,
        }
    }
}
impl Port for MemReader {
    fn read(&self) -> u32 {
        if self.read_en.read() == 1 {
            self.mem.read(self.addr.read() as usize)
        } else {
            0
        }
    }
}
