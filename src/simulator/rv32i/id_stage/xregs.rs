mod mux;
mod rgroup;
use self::rgroup::RegGroupBuilder;
use crate::common::abi::*;

use mux::Alloc as RegMuxAlloc;
use mux::Connect as RegMuxConnect;
use mux::IndexConnect as RegMuxIndexConnect;
use mux::RegMuxBuilder;
use rgroup::Alloc as RegGroupAlloc;
use rgroup::Connect as RegGroupConnect;
use rgroup::IndexAlloc as RegGroupIndexAlloc;
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
pub struct XregsBuilder {
    x: RegGroupBuilder,
    mux_rs1: RegMuxBuilder,
    mux_rs2: RegMuxBuilder,
}
impl XregsBuilder {
    pub fn new(esp: u32) -> Self {
        let mut x = RegGroupBuilder::new(esp);
        let mut mux_rs1 = RegMuxBuilder::default();
        let mut mux_rs2 = RegMuxBuilder::default();
        mux_rs1.index_connect(
            x.index_alloc(RegGroupIndexAlloc::X.into()),
            RegMuxIndexConnect::X.into(),
        );
        mux_rs2.index_connect(
            x.index_alloc(RegGroupIndexAlloc::X.into()),
            RegMuxIndexConnect::X.into(),
        );
        Self {
            x,
            mux_rs1,
            mux_rs2,
        }
    }
}
impl PortBuilder for XregsBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Rs1 => self.mux_rs1.connect(pin, RegMuxConnect::Rs.into()),
            Connect::Rs2 => self.mux_rs2.connect(pin, RegMuxConnect::Rs.into()),
            Connect::Rd => {
                self.mux_rs1.connect(pin.clone(), RegMuxConnect::Rd.into());
                self.mux_rs2.connect(pin.clone(), RegMuxConnect::Rd.into());
                self.x.connect(pin, RegGroupConnect::Rd.into());
            }
            Connect::RdData => {
                self.mux_rs1
                    .connect(pin.clone(), RegMuxConnect::RdData.into());
                self.mux_rs2
                    .connect(pin.clone(), RegMuxConnect::RdData.into());
                self.x.connect(pin, RegGroupConnect::RdData.into());
            }
            Connect::Write => {
                self.mux_rs1
                    .connect(pin.clone(), RegMuxConnect::Write.into());
                self.mux_rs2
                    .connect(pin.clone(), RegMuxConnect::Write.into());
                self.x.connect(pin, RegGroupConnect::Write.into());
            }
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::R1Data => self.mux_rs1.alloc(RegMuxAlloc::Out.into()),
            Alloc::R2Data => self.mux_rs2.alloc(RegMuxAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
}
impl ControlBuilder for XregsBuilder {
    fn build(self) -> ControlRef {
        self.x.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_regs() {
        let mut rsb = XregsBuilder::new(0);
        let mut consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(1);
        consts.push(4);
        rsb.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Rs1.into());
        rsb.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Rs2.into());
        rsb.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Rd.into());
        rsb.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Write.into());
        rsb.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::RdData.into());
        let r1 = rsb.alloc(Alloc::R1Data.into());
        let r2 = rsb.alloc(Alloc::R2Data.into());
        let rs = rsb.build();
        assert_eq!(r1.read(), 4);
        assert_eq!(r2.read(), 0);
        rs.rasing_edge();
        rs.falling_edge();
        assert_eq!(r1.read(), 4);
        assert_eq!(r2.read(), 0);
    }
}
