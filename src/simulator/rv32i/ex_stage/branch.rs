use crate::common::abi::*;
pub enum Alloc {
    BK = 0,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::BK => 0,
        }
    }
}
pub enum Connect {
    BranchType = 0,
    Op1 = 1,
    Op2 = 2,
    Jal_ = 3,
    BranchSel = 4,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::BranchType => 0,
            Connect::Op1 => 1,
            Connect::Op2 => 2,
            Connect::Jal_ => 3,
            Connect::BranchSel => 4,
        }
    }
}
#[derive(Default)]
pub struct BranchBuilder {
    inner: PortShared<Alu>,
}
impl PortBuilder for BranchBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::BK => PortRef::from(self.inner.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::BranchType => self.inner.borrow_mut().branchtype = Some(pin.clone()),
            Connect::Op1 => self.inner.borrow_mut().op1 = Some(pin.clone()),
            Connect::Op2 => self.inner.borrow_mut().op2 = Some(pin.clone()),
            Connect::Jal_ => self.inner.borrow_mut().jal_ = Some(pin.clone()),
            Connect::BranchSel => self.inner.borrow_mut().branchsel = Some(pin.clone()),
            _ => panic!("Invalid id"),
        }
    }
}

#[derive(Default)]
pub struct Alu {
    pub op1: Option<PortRef>,
    pub op2: Option<PortRef>,
    pub jal_: Option<PortRef>,
    pub branchsel: Option<PortRef>,
    pub branchtype: Option<PortRef>,
}

impl Port for Alu {
    fn read(&self) -> u32 {
        let op1 = self.op1.as_ref().unwrap().read() as i32;
        let op2 = self.op2.as_ref().unwrap().read() as i32;
        let jal_ = self.jal_.as_ref().unwrap().read();
        let branchtype = self.branchtype.as_ref().unwrap().read();
        let branchsel = self.branchsel.as_ref().unwrap().read();
        if jal_ == 1 {
            return 1;
        }
        if branchsel == 0 {
            return 0;
        }
        match branchtype & 0b111 {
            0b000 => (op1 == op2) as u32,
            0b001 => (op1 != op2) as u32,
            0b100 => (op1 < op2) as u32,
            0b101 => (op1 >= op2) as u32,
            0b110 => ((op1 as u32) < (op2 as u32)) as u32,
            0b111 => ((op1 as u32) >= (op2 as u32)) as u32,
            _ => panic!("Invalid branch type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_alu() {
        let mut alub = BranchBuilder::default();
        let mut consts = ConstsBuilder::default();
        consts.push(1);
        consts.push(2);
        consts.push(0b1);
        consts.push(0);
        consts.push(1);
        alub.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Op1);
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Op2);
        alub.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::BranchType);
        alub.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Jal_);
        alub.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::BranchSel);
        let alu = alub.alloc(Alloc::BK);
        assert_eq!(alu.read(), 1);
    }
}
