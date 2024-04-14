use crate::common::abi::*;
use crate::common::build::*;

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
    WbSel = 0,
    Npc = 1,
    AluRes = 2,
    MemData = 3,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::WbSel => 0,
            Connect::Npc => 1,
            Connect::AluRes => 2,
            Connect::MemData => 3,
        }
    }
}
#[derive(Default)]
pub struct WbStageBuilder {
    pub mux: MuxBuilder,
}

impl ControlBuilder for WbStageBuilder {
    fn build(self) -> ControlRef {
        self.mux.build()
    }
}

impl PortBuilder for WbStageBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        self.mux.alloc(id)
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        self.mux.connect(pin, id)
    }
}
