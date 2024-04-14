use crate::common::abi::*;

const PAGE_SIZE: usize = 4096;
//pagefault will allocate a new page
trait Page {
    fn get_page(&mut self, page: u32) -> Option<&mut Box<dyn Page>> {
        unreachable!("DataPage does not have subpage")
    }
    fn read(&self, offset: u32, size: u32) -> Vec<u8> {
        unreachable!("DataPage does not have subpage")
    }
    fn write(&mut self, offset: u32, data: &[u8]) {
        unreachable!("DataPage does not have subpage")
    }
}

struct DataPage {
    data: [u8; PAGE_SIZE],
}
impl Page for DataPage {
    fn read(&self, offset: u32, size: u32) -> Vec<u8> {
        self.data[offset as usize..(offset + size) as usize].to_vec()
    }
    fn write(&mut self, offset: u32, data: &[u8]) {
        self.data[offset as usize..offset as usize + data.len()].copy_from_slice(data);
    }
}
pub struct DirPage {
    pub pd2pg: [Option<Box<dyn Page>>; 512],
}
const EMPTY_PAGE: std::option::Option<std::boxed::Box<(dyn Page + 'static)>> = None;
impl Default for DirPage {
    fn default() -> Self {
        Self {
            pd2pg: [EMPTY_PAGE; 512],
        }
    }
}
impl Page for DirPage {
    fn get_page(&mut self, page: u32) -> Option<&mut Box<dyn Page>> {
        if self.pd2pg[page as usize].is_none() {
            self.pd2pg[page as usize] = Some(Box::new(DataPage {
                data: [0; PAGE_SIZE],
            }));
        }
        self.pd2pg[page as usize].as_mut()
    }
}
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    Address = 0,
    Input = 1,
    Write = 2,
    Read = 3,
}
#[derive(Default)]
pub struct MemBuilder {
    pub inner: ControlShared<Mem>,
}
impl MemBuilder {
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            inner: ControlShared::new(Mem {
                data: memory,
                ..Default::default()
            }),
        }
    }
}
impl ControlBuilder for MemBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for MemBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    // Connect the address and input pin
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Address => self.inner.borrow_mut().address = Some(pin),
            Self::Connect::Input => self.inner.borrow_mut().input = Some(pin),
            Self::Connect::Write => self.inner.borrow_mut().write = Some(pin),
            Self::Connect::Read => self.inner.borrow_mut().read = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    // alloc the id for the memory
    // 0 for address
    // 1 for input
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        self.inner.clone().into_shared().into()
    }
}
#[derive(Default)]
pub struct Mem {
    pub id: usize,
    data: Vec<u8>,
    stack: Vec<u8>,
    pub input: Option<PortRef>,
    pub input_cache: u32,
    pub write: Option<PortRef>,
    pub write_cache: u32,
    pub read: Option<PortRef>,
    pub address: Option<PortRef>,
    pub address_cache: usize,
}
const STACK_ADDR: u32 = 0x7FFFFFF0;
impl Mem {
    // pub fn new() -> Self {
    //     let mut data = DirPage::default();
    //     data.pd2pg.iter_mut().for_each(|x| {
    //         *x = Some(Box::new(DirPage::default()));
    //     });
    //     Self {
    //         data,
    //         ..Default::default()
    //     }
    // }
    pub fn store(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}
impl Port for Mem {
    fn read(&self) -> u32 {
        if self
            .read
            .as_ref()
            .expect("read enable is not connected")
            .read()
            != 1
        {
            return 0;
        }
        let addr = self
            .address
            .as_ref()
            .expect("address is not connected")
            .read() as usize;
        let (arr, addr) = if addr > STACK_ADDR as usize {
            (&self.stack, addr - STACK_ADDR as usize)
        } else {
            (&self.data, addr)
        };
        if addr + 4 > arr.len() {
            return 0;
        }
        u32::from_ne_bytes([arr[addr], arr[addr + 1], arr[addr + 2], arr[addr + 3]])
    }
}
impl Control for Mem {
    fn rasing_edge(&mut self) {
        if self
            .write
            .as_ref()
            .expect("write enable is not connected")
            .read()
            != 1
        {
            self.write_cache = 0;
            return;
        }
        self.address_cache = self
            .address
            .as_ref()
            .expect("address is not connected")
            .read() as usize;
        self.input_cache = self.input.as_ref().expect("input is not connected").read();
        self.write_cache = 1;
    }
    fn falling_edge(&mut self) {
        if self.write_cache == 1 {
            let (arr, addr) = if self.address_cache > STACK_ADDR as usize {
                (&mut self.stack, self.address_cache - STACK_ADDR as usize)
            } else {
                (&mut self.data, self.address_cache)
            };
            if addr + 4 > arr.len() {
                arr.resize(addr + 4, 0);
            }
            arr[addr] = (self.input_cache & 0xff) as u8;
            arr[addr + 1] = ((self.input_cache >> 8) & 0xff) as u8;
            arr[addr + 2] = ((self.input_cache >> 16) & 0xff) as u8;
            arr[addr + 3] = ((self.input_cache >> 24) & 0xff) as u8;
        }
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!("mem: {:#X}", self.read())
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
        let mut tb = MemBuilder::new(b"12345678".to_vec());
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(1);
        let mut ab = AddBuilder::default();
        let add = ab.alloc(AddAlloc::Out);
        ab.connect(constant.alloc(ConstsAlloc::Out(0)), AddConnect::In(0));
        let mut rb = RegBuilder::new(0);
        rb.connect(ab.alloc(AddAlloc::Out), RegConnect::In);
        rb.connect(
            constant.alloc(ConstsAlloc::Out(1)),
            RegConnect::Enable.into(),
        );
        ab.connect(rb.alloc(RegAlloc::Out), AddConnect::In(0));
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::Address);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::Input);
        tb.connect(rb.alloc(RegAlloc::Out), MemConnect::Write);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MemConnect::Read.into());
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
        let mut tb = MemBuilder::default();
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        constant.push(3);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MemConnect::Address);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MemConnect::Input);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::Write);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::Read);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MemConnect::Read);
    }
}
