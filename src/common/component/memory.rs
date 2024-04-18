mod mem;
mod read;
mod write;
use crate::common::abi::*;
use mem::Mem;

use self::read::MemReader;
use self::write::MemWriter;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    Addr = 0,
    Data = 1,
    WriteEn = 2,
    ReadEn = 3,
}

pub struct MemBuilder {
    pub writer: MemWriter,
    pub reader: PortShared<MemReader>,
}
impl MemBuilder {
    pub fn new() -> Self {
        Self::with_data(0, vec![])
    }
    pub fn with_data(addr: usize, data: Vec<u8>) -> Self {
        let mem = IndexPortShared::new(Mem::with_data(addr, data));
        Self {
            writer: MemWriter::new(mem.clone()),
            reader: PortShared::new(MemReader::new(mem.into())),
        }
    }
}
impl Default for MemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ControlBuilder for MemBuilder {
    fn build(self) -> ControlRef {
        self.writer.into()
    }
}
impl PortBuilder for MemBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    // Connect the address and input pin
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Addr => {
                self.writer.addr = pin.clone();
                self.reader.borrow_mut().addr = pin;
            }
            Self::Connect::Data => self.writer.write_data = pin,
            Self::Connect::WriteEn => self.writer.write_en = pin,
            Self::Connect::ReadEn => self.reader.borrow_mut().read_en = pin,
        }
    }
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        PortRef::from(self.reader.clone())
    }
}

pub mod build {
    pub use super::Alloc as MemAlloc;
    pub use super::Connect as MemConnect;
    pub use super::MemBuilder;
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_mem() {
        let mut tb = MemBuilder::with_data(0, b"12345678".to_vec());
        let mut constant = ConstsBuilder::default();
        let mut ab = AddBuilder::default();
        let add = ab.alloc(AddAlloc::Out);
        ab.connect(constant.alloc(ConstsAlloc::Out(1)), AddConnect::In(0));
        let mut rb = RegBuilder::new(0);
        rb.connect(ab.alloc(AddAlloc::Out), RegConnect::In);
        rb.connect(constant.alloc(ConstsAlloc::Out(1)), RegConnect::Enable);
        rb.connect(constant.alloc(ConstsAlloc::Out(0)), RegConnect::Clear);
        ab.connect(rb.alloc(RegAlloc::Out), AddConnect::In(1));
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::Addr);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::Data);
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::WriteEn);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::ReadEn);
        let t = tb.alloc(MemAlloc::Out);
        let tc = tb.build();
        let rc = rb.build();
        tc.rasing_edge();
        // println!("{:#X}", t.borrow().read(0).unwrap());
        assert_eq!(t.read(), u32::from_ne_bytes([0x31, 0x32, 0x33, 0x34]));
        tc.falling_edge();
        assert_eq!(t.read(), u32::from_ne_bytes([0x31, 0x32, 0x33, 0x34]));
        rc.rasing_edge();
        assert_eq!(add.read(), 1);
        rc.falling_edge();
        assert_eq!(add.read(), 2);
        tc.rasing_edge();
        tc.falling_edge();
        assert_eq!(t.read(), u32::from_ne_bytes([1, 0, 0, 0]));
    }
}
