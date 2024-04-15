use crate::common::abi::*;

pub enum Alloc {
    Out = 0,
}

pub enum Connect {
    Rs = 0,
    Rd = 1,
    RdData = 2,
    Write = 3,
}

pub enum IndexConnect {
    X,
}
pub enum IndexAlloc {}

#[derive(Default)]
pub struct RegMuxBuilder {
    rs: Option<PortRef>,
    rd: Option<PortRef>,
    rd_data: Option<PortRef>,
    write: Option<PortRef>,
    x: Option<IndexPortRef>,
}
impl PortBuilder for RegMuxBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Rs => self.rs = Some(pin),
            Connect::Rd => self.rd = Some(pin),
            Connect::RdData => self.rd_data = Some(pin),
            Connect::Write => self.write = Some(pin),
        }
    }
    fn alloc(&mut self, _: Alloc) -> PortRef {
        PortRef::from(PortShared::new(RegMux {
            rs: self.rs.clone().unwrap(),
            rd: self.rd.clone().unwrap(),
            rd_data: self.rd_data.clone().unwrap(),
            write: self.write.clone().unwrap(),
            x: self.x.clone().unwrap(),
        }))
    }
}
impl IndexPortBuilder for RegMuxBuilder {
    type IndexAlloc = IndexAlloc;
    type IndexConnect = IndexConnect;

    fn index_connect(&mut self, pin: IndexPortRef, id: IndexConnect) {
        match id {
            IndexConnect::X => self.x = Some(pin),
        }
    }
    fn index_alloc(&mut self, _id: IndexAlloc) -> IndexPortRef {
        unreachable!("RegMux has no index output")
    }
}

#[derive(Debug)]
struct RegMux {
    rs: PortRef,
    rd: PortRef,
    rd_data: PortRef,
    write: PortRef,
    x: IndexPortRef,
}
impl Port for RegMux {
    fn read(&self) -> u32 {
        let rs = self.rs.read();
        let rd = self.rd.read();
        let write = self.write.read();
        if write == 1 && rd == rs {
            self.rd_data.read()
        } else {
            self.x.read(rs as usize)
        }
    }
}
