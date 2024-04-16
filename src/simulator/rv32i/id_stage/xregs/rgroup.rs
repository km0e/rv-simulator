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
            rd_cache: 0,
            rd_data: self.rd_data.unwrap(),
            rd_data_cache: 0,
            write: self.write.unwrap(),
            x: self.x.clone(),
        })
        .into()
    }
}
impl PortBuilder for RegGroupBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Rd => self.rd = Some(pin),
            Connect::RdData => self.rd_data = Some(pin),
            Connect::Write => self.write = Some(pin),
        }
    }
    fn alloc(&mut self, _id: Alloc) -> PortRef {
        unreachable!("RegGroup has no output")
    }
}
impl IndexPortBuilder for RegGroupBuilder {
    type IndexAlloc = IndexAlloc;
    type IndexConnect = IndexConnect;
    fn index_connect(&mut self, _pin: IndexPortRef, _id: IndexConnect) {
        unreachable!("RegGroup has no index output")
    }
    fn index_alloc(&mut self, id: IndexAlloc) -> IndexPortRef {
        match id {
            IndexAlloc::X => self.x.shared().into(),
        }
    }
}
#[derive(Debug)]
struct RegGroupControl {
    rd: PortRef,
    rd_cache: u32,
    rd_data: PortRef,
    rd_data_cache: u32,
    write: PortRef,
    x: IndexPortShared<RegGroup>,
}
impl Control for RegGroupControl {
    fn rasing_edge(&mut self) {
        if self.write.read() != 1 {
            self.rd_cache = 0;
            return;
        }
        self.rd_cache = self.rd.read();
        self.rd_data_cache = self.rd_data.read();
    }
    fn falling_edge(&mut self) {
        if self.rd_cache != 0 {
            self.x.borrow_mut().x[self.rd_cache as usize] = self.rd_data_cache;
        }
    }
    fn input(&self) -> Vec<(String, u32)> {
        vec![
            ("rd".to_string(), self.rd.read()),
            ("rd_data".to_string(), self.rd_data.read()),
            ("write".to_string(), self.write.read()),
        ]
    }
    fn output(&self) -> Vec<(String, u32)> {
        vec![]
    }
}

#[derive(Debug)]
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
