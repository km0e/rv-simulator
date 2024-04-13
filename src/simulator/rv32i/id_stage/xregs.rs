use crate::component::{Control, ControlRef, ControlShared};

use super::{Builder, Lat, PortRef, PortShared};
pub enum Alloc {
    R1Data = 0,
    R2Data = 1,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::R1Data => 0,
            Alloc::R2Data => 1,
        }
    }
}
pub enum Connect {
    Rs1 = 0,
    Rs2 = 1,
    Rd = 2,
    RdData = 3,
    Write = 4,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Rs1 => 0,
            Connect::Rs2 => 1,
            Connect::Rd => 2,
            Connect::RdData => 3,
            Connect::Write => 4,
        }
    }
}
#[derive(Default)]
pub struct XregsBuilder {
    inner: ControlShared<Regs>,
}
impl XregsBuilder {
    pub fn new(esp: u32) -> Self {
        Self {
            inner: ControlShared::new(Regs::new(esp)),
        }
    }
}
impl Builder for XregsBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().rs1 = Some(pin),
            1 => self.inner.borrow_mut().rs2 = Some(pin),
            2 => self.inner.borrow_mut().rd = Some(pin),
            3 => self.inner.borrow_mut().rd_data = Some(pin),
            4 => self.inner.borrow_mut().write = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => PortRef::from(self.inner.borrow().r1data.clone()),
            1 => PortRef::from(self.inner.borrow().r2data.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(self.inner.clone()))
    }
}
#[derive(Default)]
pub struct Regs {
    pub rs1: Option<PortRef>,
    pub rs1_val: u32,
    pub rs2: Option<PortRef>,
    pub rs2_val: u32,
    pub rd: Option<PortRef>,
    pub rd_data: Option<PortRef>,
    pub write: Option<PortRef>,
    pub x: [u32; 32],
    pub r1data: PortShared<Lat>,
    pub r2data: PortShared<Lat>,
}
impl Regs {
    pub fn new(esp: u32) -> Self {
        let mut slf = Self {
            x: [0; 32],
            r1data: PortShared::new(Lat::new(0)),
            r2data: PortShared::new(Lat::new(0)),
            ..Default::default()
        };
        slf.x[2] = esp;
        slf
    }
}
impl Control for Regs {
    fn rasing_edge(&mut self) {
        match self.rs1 {
            Some(ref rs1) => self.rs1_val = rs1.read(),
            None => {
                unimplemented!()
            }
        };
        match self.rs2 {
            Some(ref rs2) => self.rs2_val = rs2.read(),
            None => {
                unimplemented!()
            }
        };
        if match self.write {
            Some(ref enable) => enable.read() == 1,
            None => {
                unimplemented!()
            }
        } {
            match (&self.rd, &self.rd_data) {
                (Some(ref rd), Some(ref rd_data)) => {
                    self.x[rd.read() as usize] = rd_data.read();
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }
    fn falling_edge(&mut self) {
        self.r1data.borrow_mut().data = self.x[self.rs1_val as usize];
        self.r2data.borrow_mut().data = self.x[self.rs2_val as usize];
    }
    fn debug(&self) -> String {
        format!(
            "regs: r1data: {:#X}, r2data: {:#X}",
            self.r1data.borrow().data,
            self.r2data.borrow().data
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::*;

    #[test]
    fn test_regs() {
        let mut rsb = XregsBuilder::default();
        let mut consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(1);
        consts.push(2);
        consts.push(0);
        consts.push(4);
        rsb.connect(consts.alloc(0), 0);
        rsb.connect(consts.alloc(2), 1);
        rsb.connect(consts.alloc(3), 2);
        rsb.connect(consts.alloc(4), 3);
        rsb.connect(consts.alloc(1), 4);
        consts.build();
        let r1 = rsb.alloc(0);
        let r2 = rsb.alloc(1);
        let rs = rsb.build().unwrap();
        assert_eq!(r1.read(), 0);
        assert_eq!(r2.read(), 0);
        rs.rasing_edge();
        rs.falling_edge();
        assert_eq!(r1.read(), 4);
        assert_eq!(r2.read(), 0);
    }
}
