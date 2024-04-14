use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    WbSel = 0,
    Npc = 1,
    AluRes = 2,
    MemData = 3,
}
#[derive(Default)]
pub struct WbStageBuilder {
    pub mux: MuxBuilder,
}

impl PortBuilder for WbStageBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        self.mux.alloc(MuxAlloc::Out)
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::WbSel => self.mux.connect(pin, MuxConnect::Select),
            Connect::Npc => self.mux.connect(pin, MuxConnect::In(0)),
            Connect::AluRes => self.mux.connect(pin, MuxConnect::In(1)),
            Connect::MemData => self.mux.connect(pin, MuxConnect::In(2)),
        }
    }
}
