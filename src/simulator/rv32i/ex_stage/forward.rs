use crate::common::abi::*;

pub enum Alloc {
    Forward1 = 0,
    Forward2 = 1,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Forward1 => 0,
            Alloc::Forward2 => 1,
        }
    }
}
pub enum Connect {
    Rs1 = 0,
    Rs2 = 1,
    RdMem = 2,
    RdMemWrite = 3,
    RdWb = 4,
    RdWbWrite = 5,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Rs1 => 0,
            Connect::Rs2 => 1,
            Connect::RdMem => 2,
            Connect::RdMemWrite => 3,
            Connect::RdWb => 4,
            Connect::RdWbWrite => 5,
        }
    }
}
#[derive(Default)]
pub struct ForwardBuilder {
    pub forward1: PortShared<Forward>,
    pub forward2: PortShared<Forward>,
}
impl PortBuilder for ForwardBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::Forward1 => PortRef::from(self.forward1.clone()),
            Alloc::Forward2 => PortRef::from(self.forward2.clone()),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Rs1 => self.forward1.borrow_mut().rs = Some(pin.clone()),
            Connect::Rs2 => self.forward2.borrow_mut().rs = Some(pin.clone()),
            Connect::RdMem => {
                self.forward1.borrow_mut().rd_mem = Some(pin.clone());
                self.forward2.borrow_mut().rd_mem = Some(pin.clone());
            }
            Connect::RdMemWrite => {
                self.forward1.borrow_mut().rd_mem_write = Some(pin.clone());
                self.forward2.borrow_mut().rd_mem_write = Some(pin.clone());
            }
            Connect::RdWb => {
                self.forward1.borrow_mut().rd_wb = Some(pin.clone());
                self.forward2.borrow_mut().rd_wb = Some(pin.clone());
            }
            Connect::RdWbWrite => {
                self.forward1.borrow_mut().rd_wb_write = Some(pin.clone());
                self.forward2.borrow_mut().rd_wb_write = Some(pin.clone());
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Forward {
    pub rs: Option<PortRef>,
    pub rd_mem: Option<PortRef>,
    pub rd_mem_write: Option<PortRef>,
    pub rd_wb: Option<PortRef>,
    pub rd_wb_write: Option<PortRef>,
}

impl Port for Forward {
    fn read(&self) -> u32 {
        let rs = match self.rs {
            Some(ref rs) => rs.read(),
            None => {
                unimplemented!()
            }
        };
        let rd_mem = match self.rd_mem {
            Some(ref rd_mem) => rd_mem.read(),
            None => {
                unimplemented!()
            }
        };
        let rd_mem_write = match self.rd_mem_write {
            Some(ref rd_mem_write) => rd_mem_write.read(),
            None => {
                unimplemented!()
            }
        };
        let rd_wb = match self.rd_wb {
            Some(ref rd_wb) => rd_wb.read(),
            None => {
                unimplemented!()
            }
        };
        let rd_wb_write = match self.rd_wb_write {
            Some(ref rd_wb_write) => rd_wb_write.read(),
            None => {
                unimplemented!()
            }
        };
        if rs == rd_mem && rd_mem_write == 1 {
            1
        } else if rs == rd_wb && rd_wb_write == 1 {
            2
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[allow(clippy::too_many_arguments)]
    fn build_test(
        rs1: u32,
        rs2: u32,
        rd_mem: u32,
        rd_mem_write: u32,
        rd_wb: u32,
        rd_wb_write: u32,
        forward1: u32,
        forward2: u32,
    ) {
        let mut forward = ForwardBuilder::default();
        let mut consts = ConstsBuilder::default();
        consts.push(rs1);
        consts.push(rs2);
        consts.push(rd_mem);
        consts.push(rd_mem_write);
        consts.push(rd_wb);
        consts.push(rd_wb_write);
        forward.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Rs1);
        forward.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Rs2);
        forward.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::RdMem);
        forward.connect(consts.alloc(ConstsAlloc::Out(3)), Connect::RdMemWrite);
        forward.connect(consts.alloc(ConstsAlloc::Out(4)), Connect::RdWb);
        forward.connect(consts.alloc(ConstsAlloc::Out(5)), Connect::RdWbWrite);
        let forward1_ = forward.alloc(Alloc::Forward1);
        let forward2_ = forward.alloc(Alloc::Forward2);
        assert_eq!(forward1_.read(), forward1);
        assert_eq!(forward2_.read(), forward2);
    }
    #[test]
    fn test_forward() {
        build_test(0, 8, 0xc, 0, 8, 1, 0, 2);
    }
}
