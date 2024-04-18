use crate::common::abi::*;
pub enum Alloc {
    BK = 0,
}
pub enum Connect {
    BranchType = 0,
    Op1 = 1,
    Op2 = 2,
    Jal_ = 3,
    BranchSel = 4,
}
#[derive(Default)]
pub struct BranchBuilder {
    inner: ControlShared<Branch>,
}
impl ControlBuilder for BranchBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for BranchBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::BK => self.inner.clone().into_shared().into(),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::BranchType => self.inner.borrow_mut().branchtype = Some(pin.clone()),
            Connect::Op1 => self.inner.borrow_mut().op1 = Some(pin.clone()),
            Connect::Op2 => self.inner.borrow_mut().op2 = Some(pin.clone()),
            Connect::Jal_ => self.inner.borrow_mut().jal_ = Some(pin.clone()),
            Connect::BranchSel => self.inner.borrow_mut().branchsel = Some(pin.clone()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Branch {
    pub op1: Option<PortRef>,
    pub op2: Option<PortRef>,
    pub jal_: Option<PortRef>,
    pub branchsel: Option<PortRef>,
    pub branchtype: Option<PortRef>,
}
impl Control for Branch {
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("br", self.read())]
    }
}
impl Port for Branch {
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
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Op1);
        alub.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::Op2);
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::BranchType);
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Jal_);
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::BranchSel);
        let alu = alub.alloc(Alloc::BK);
        assert_eq!(alu.read(), 1);
    }
}
