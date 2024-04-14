use crate::common::abi::*;

pub enum Alloc {
    Out = 0,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Out => 0,
        }
    }
}
pub enum Connect {
    Rs = 0,
    Rd = 1,
    RdData = 2,
    Write = 3,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Rs => 0,
            Connect::Rd => 1,
            Connect::RdData => 2,
            Connect::Write => 3,
        }
    }
}
pub enum IndexConnect {
    X = 0,
}
impl From<IndexConnect> for usize {
    fn from(alloc: IndexConnect) -> usize {
        match alloc {
            IndexConnect::X => 0,
        }
    }
}
#[derive(Default)]
pub struct RegMuxBuilder {
    rs: Option<PortRef>,
    rd: Option<PortRef>,
    rd_data: Option<PortRef>,
    write: Option<PortRef>,
    x: Option<IndexPortRef>,
}
impl PortBuilder for RegMuxBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.rs = Some(pin),
            1 => self.rd = Some(pin),
            2 => self.rd_data = Some(pin),
            3 => self.write = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, _: usize) -> PortRef {
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
    fn index_connect(&mut self, pin: IndexPortRef, id: usize) {
        match id {
            0 => self.x = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn index_alloc(&mut self, _id: usize) -> IndexPortRef {
        unreachable!("RegMux has no index output")
    }
}

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
