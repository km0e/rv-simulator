use std::{cell::RefCell, collections::HashMap};

use crate::common::abi::*;
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
    write_en: Option<PortRef>,
    read_en: Option<PortRef>,
    addr: Option<PortRef>,
    write_data: Option<PortRef>,
    mem: IndexPortShared<Mem>,
    reader: Option<MemReader>,
}
impl MemBuilder {
    pub fn new() -> Self {
        Self {
            mem: IndexPortShared::new(Mem {
                data: HashMap::new(),
            }),
            write_en: None,
            read_en: None,
            addr: None,
            write_data: None,
            reader: None,
        }
    }
    pub fn with_data(addr: usize, data: Vec<u8>) -> Self {
        Self {
            mem: IndexPortShared::new(Mem::with_data(addr, data)),
            write_en: None,
            read_en: None,
            addr: None,
            write_data: None,
            reader: None,
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
        ControlShared::new(MemWriter {
            write_en: self.write_en.expect("write enable is not connected"),
            write_en_cache: 0,
            addr: self.addr.expect("address is not connected"),
            addr_cache: 0,
            write_data: self.write_data.expect("write data is not connected"),
            write_data_cache: 0,
            mem: self.mem,
        })
        .into()
    }
}
impl PortBuilder for MemBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    // Connect the address and input pin
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Addr => self.addr = Some(pin),
            Self::Connect::Data => self.write_data = Some(pin),
            Self::Connect::WriteEn => self.write_en = Some(pin),
            Self::Connect::ReadEn => self.read_en = Some(pin),
        }
    }
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        match self.reader {
            Some(ref mut reader) => reader.clone().into(),
            None => {
                let reader = MemReader {
                    read_en: self.read_en.clone().expect("read enable is not connected"),
                    addr: self.addr.clone().expect("address is not connected"),
                    mem: self.mem.clone().into(),
                };
                self.reader = Some(reader.clone());
                reader.into()
            }
        }
    }
}
const PAGE_SIZE: usize = 0x1000;

#[derive(Debug)]
pub struct Mem {
    data: HashMap<usize, [u8; PAGE_SIZE]>,
}
impl Mem {
    pub fn with_data(addr: usize, data: Vec<u8>) -> Self {
        let mut mem = HashMap::new();
        let mut start = 0;
        while start + PAGE_SIZE < data.len() {
            let mut page = [0; PAGE_SIZE];
            page.copy_from_slice(&data[start..start + PAGE_SIZE]);
            mem.insert(addr + start / PAGE_SIZE, page);
            start += PAGE_SIZE;
        }
        let mut page = [0; PAGE_SIZE];
        page[..data.len() - start].copy_from_slice(&data[start..]);
        mem.insert(addr + start / PAGE_SIZE, page);

        Self { data: mem }
    }
}
impl Mem {
    pub fn write(&mut self, addr: usize, data: u32) {
        let page = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        let page = self.data.entry(page).or_insert_with(|| [0; PAGE_SIZE]);
        page[offset] = (data & 0xff) as u8;
        page[offset + 1] = ((data >> 8) & 0xff) as u8;
        page[offset + 2] = ((data >> 16) & 0xff) as u8;
        page[offset + 3] = ((data >> 24) & 0xff) as u8;
    }
}
impl IndexPort for Mem {
    fn read(&self, addr: usize) -> u32 {
        let page = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        if let Some(page) = self.data.get(&page) {
            u32::from_ne_bytes([
                page[offset],
                page[offset + 1],
                page[offset + 2],
                page[offset + 3],
            ])
        } else {
            0
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemReader {
    read_en: PortRef,
    addr: PortRef,
    mem: IndexPortRef,
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
#[derive(Debug)]
pub struct MemWriter {
    write_en: PortRef,
    write_en_cache: u32,
    addr: PortRef,
    addr_cache: u32,
    write_data: PortRef,
    write_data_cache: u32,
    mem: IndexPortShared<Mem>,
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
        constant.push(1);
        constant.push(1);
        let mut ab = AddBuilder::default();
        let add = ab.alloc(AddAlloc::Out);
        ab.connect(constant.alloc(ConstsAlloc::Out(0)), AddConnect::In(0));
        let mut rb = RegBuilder::new(0);
        rb.connect(ab.alloc(AddAlloc::Out), RegConnect::In);
        rb.connect(constant.alloc(ConstsAlloc::Out(1)), RegConnect::Enable);
        ab.connect(rb.alloc(RegAlloc::Out), AddConnect::In(0));
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::Addr);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::Data);
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::WriteEn);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MemConnect::ReadEn);
        let t = tb.alloc(MemAlloc::Out);
        let tc = tb.build();
        let rc = rb.build();
        tc.rasing_edge();
        // println!("{:#X}", t.borrow().read(0).unwrap());
        assert_eq!(t.read(), u32::from_ne_bytes([0, 0, 0, 0]));
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
    #[test]
    #[should_panic]
    fn test_mem_panic() {
        let mut tb = MemBuilder::with_data(0, vec![]);
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        constant.push(3);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MemConnect::Addr);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::Data);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::WriteEn);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::ReadEn);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::ReadEn);
    }
}
