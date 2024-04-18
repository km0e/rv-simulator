use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Pc = 0,
    Npc = 1,
    Imem = 2,
}

pub enum Connect {
    PcEnable = 0,
    NpcSel = 1,
    Npc = 2,
}
pub struct IfStageBuilder {
    pub npc_mux: MuxBuilder,
    pub pc: RegBuilder,
    pub add: AddBuilder,
    pub imem: MemBuilder,
}
impl IfStageBuilder {
    pub fn new(entry: u32, instruction_memory: Vec<u8>) -> Self {
        // add if stage
        let mut consts = ConstsBuilder::default();
        let mut if_pc_inc = MuxBuilder::default();
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(4)), MuxConnect::In(0));
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(2)), MuxConnect::In(1));
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(0)), MuxConnect::Select);
        let mut pc = RegBuilder::new(entry);
        pc.connect(consts.alloc(ConstsAlloc::Out(0)), RegConnect::Clear);
        let mut add = AddBuilder::default();
        add.connect(if_pc_inc.alloc(MuxAlloc::Out), AddConnect::In(0));
        let mut imem = MemBuilder::with_data(0, instruction_memory);
        imem.connect(consts.alloc(ConstsAlloc::Out(0)), MemConnect::WriteEn);
        imem.connect(consts.alloc(ConstsAlloc::Out(1)), MemConnect::ReadEn);
        imem.connect(consts.alloc(ConstsAlloc::Out(1)), MemConnect::Data);
        let mut npc_mux = MuxBuilder::default();
        npc_mux.connect(add.alloc(AddAlloc::Out), MuxConnect::In(0));
        pc.connect(npc_mux.alloc(MuxAlloc::Out), RegConnect::In);

        IfStageBuilder {
            npc_mux,
            pc,
            add,
            imem,
        }
    }
}
impl ControlBuilder for IfStageBuilder {
    fn build(self) -> ControlRef {
        IfStage {
            pc: self.pc.build(),
            npc_mux: self.npc_mux.build(),
            imem: self.imem.build(),
        }
        .into()
    }
}
impl PortBuilder for IfStageBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::Pc => self.pc.alloc(RegAlloc::Out),
            Alloc::Npc => self.add.alloc(AddAlloc::Out),
            Alloc::Imem => self.imem.alloc(MemAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::PcEnable => {
                self.pc.connect(pin, RegConnect::Enable);
                self.add
                    .connect(self.pc.alloc(RegAlloc::Out), AddConnect::In(1));
                self.imem
                    .connect(self.pc.alloc(RegAlloc::Out), MemConnect::Addr);
            }
            Connect::NpcSel => self.npc_mux.connect(pin, MuxConnect::Select),
            Connect::Npc => self.npc_mux.connect(pin, MuxConnect::In(1)),
        }
    }
}
#[derive(Debug)]
pub struct IfStage {
    pub pc: ControlRef,
    pub npc_mux: ControlRef,
    pub imem: ControlRef,
}
impl Control for IfStage {
    fn rasing_edge(&mut self) {
        self.pc.rasing_edge();
        self.imem.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.pc.falling_edge();
        self.imem.falling_edge();
    }
    fn inner_signal(&self) -> Vec<(&'static str, u32)> {
        let mut res = vec![];
        res.extend(self.npc_mux.output());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_if() {
        let text = b"abcdefgh".to_vec();
        let mut ifb = IfStageBuilder::new(0, text.to_vec());
        let mut consts = ConstsBuilder::default();
        ifb.npc_mux
            .connect(consts.alloc(ConstsAlloc::Out(0)), MuxConnect::Select);
        ifb.imem
            .connect(consts.alloc(ConstsAlloc::Out(0)), MemConnect::WriteEn);
        ifb.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::PcEnable);
        let pc = ifb.pc.alloc(RegAlloc::Out);
        let npc = ifb.add.alloc(AddAlloc::Out);
        let imem = ifb.imem.alloc(MemAlloc::Out);
        let if_ = ifb.build();
        assert_eq!(pc.read(), 0);
        assert_eq!(npc.read(), 4);
        if_.rasing_edge();
        assert_eq!(pc.read(), 0);
        assert_eq!(npc.read(), 4);
        if_.falling_edge();
        assert_eq!(pc.read(), 4);
        assert_eq!(npc.read(), 8);
        assert_eq!(imem.read(), u32::from_ne_bytes([b'e', b'f', b'g', b'h']));
    }
}
