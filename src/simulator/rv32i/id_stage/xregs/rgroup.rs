use crate::common::abi::*;

pub enum Alloc {}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {}
    }
}
pub enum IndexAlloc {
    X,
}
impl From<IndexAlloc> for usize {
    fn from(alloc: IndexAlloc) -> usize {
        match alloc {
            IndexAlloc::X => 0,
        }
    }
}
pub enum Connect {
    Rd,
    RdData,
    Write,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Rd => 0,
            Connect::RdData => 1,
            Connect::Write => 2,
        }
    }
}
pub enum IndexConnect {}

impl From<IndexConnect> for usize {
    fn from(alloc: IndexConnect) -> usize {
        match alloc {}
    }
}
pub struct RegGroupBuilder {
    pub rd: Option<PortRef>,
    pub rd_data: Option<PortRef>,
    pub write: Option<PortRef>,
    x: IndexPortShared<RegGroup>,
}
impl RegGroupBuilder {
    pub fn new(esp: u32) -> Self {
        Self {
            rd: None,
            rd_data: None,
            write: None,
            x: IndexPortShared::new(RegGroup::new(esp)),
        }
    }
}
impl ControlBuilder for RegGroupBuilder {
    fn build(self) -> ControlRef {
        ControlShared::new(RegGroupControl {
            rd: self.rd.unwrap(),
            rd_data: self.rd_data.unwrap(),
            write: self.write.unwrap(),
            x: self.x.clone(),
        })
        .into()
    }
}
impl PortBuilder for RegGroupBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.rd = Some(pin),
            1 => self.rd_data = Some(pin),
            2 => self.write = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, _id: usize) -> PortRef {
        unreachable!("RegGroup has no output")
    }
}
impl IndexPortBuilder for RegGroupBuilder {
    fn index_connect(&mut self, pin: IndexPortRef, id: usize) {
        unreachable!("RegGroup has no index output")
    }
    fn index_alloc(&mut self, id: usize) -> IndexPortRef {
        match id {
            0 => self.x.shared().into(),
            _ => unreachable!("RegGroup has no index > 0 output"),
        }
    }
}
struct RegGroupControl {
    rd: PortRef,
    rd_data: PortRef,
    write: PortRef,
    x: IndexPortShared<RegGroup>,
}
impl Control for RegGroupControl {
    fn rasing_edge(&mut self) {
        if self.write.read() == 1 {
            self.x.borrow_mut().x[self.rd.read() as usize] = self.rd_data.read();
        }
    }
    fn falling_edge(&mut self) {}
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!("regs: rd_data: {:#X}", self.rd_data.read())
    }
}

struct RegGroup {
    x: [u32; 32],
}
impl RegGroup {
    fn new(esp: u32) -> Self {
        let mut x = [0; 32];
        x[2] = esp;
        Self { x }
    }
}
impl IndexPort for RegGroup {
    fn read(&self, index: usize) -> u32 {
        self.x[index]
    }
}
