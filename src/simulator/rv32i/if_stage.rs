use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Pc = 0,
    Npc = 1,
    Imem = 2,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Pc => 0,
            Alloc::Npc => 1,
            Alloc::Imem => 2,
        }
    }
}

pub enum Connect {
    PcEnable = 0,
    NpcSel = 1,
    Npc = 2,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::PcEnable => 0,
            Connect::NpcSel => 1,
            Connect::Npc => 2,
        }
    }
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
        // set up consts
        let mut consts = ConstsBuilder::default();
        consts.push(4);
        consts.push(2);
        consts.push(0);
        consts.push(1);
        // set up pc increment mux
        let mut if_pc_inc = MuxBuilder::default();
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(0)), MuxConnect::In(0));
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(1)), MuxConnect::In(1));
        if_pc_inc.connect(consts.alloc(ConstsAlloc::Out(2)), MuxConnect::Select);
        // set up npc mux
        let mut if_npc_mux = MuxBuilder::default();
        // set up pc
        let mut if_pc = RegBuilder::new(entry);
        if_pc.connect(if_npc_mux.alloc(MuxAlloc::Out), RegConnect::In);
        // set up add
        let mut if_add = AddBuilder::new();
        if_add.connect(if_pc_inc.alloc(MuxAlloc::Out), AddConnect::In(0));
        if_add.connect(if_pc.alloc(RegAlloc::Out), AddConnect::In(1));
        // connect npc and add
        if_npc_mux.connect(if_add.alloc(AddAlloc::Out), MuxConnect::In(0));
        // set up instruction memory
        let mut if_imem = MemBuilder::with_data(0, instruction_memory);
        if_imem.connect(if_pc.alloc(RegAlloc::Out), MemConnect::Addr);
        if_imem.connect(consts.alloc(ConstsAlloc::Out(2)), MemConnect::WriteEn);
        if_imem.connect(consts.alloc(ConstsAlloc::Out(3)), MemConnect::ReadEn);
        if_imem.connect(consts.alloc(ConstsAlloc::Out(3)), MemConnect::Data);
        //cache
        // build if stage
        IfStageBuilder {
            npc_mux: if_npc_mux,
            pc: if_pc,
            add: if_add,
            imem: if_imem,
        }
    }
}
impl ControlBuilder for IfStageBuilder {
    fn build(self) -> ControlRef {
        IfStage {
            pc: self.pc.build(),
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
            Connect::PcEnable => self.pc.connect(pin, RegConnect::Enable),
            Connect::NpcSel => self.npc_mux.connect(pin, MuxConnect::Select),
            Connect::Npc => self.npc_mux.connect(pin, MuxConnect::In(1)),
        }
    }
}
#[derive(Debug)]
pub struct IfStage {
    pub pc: ControlRef,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_if() {
        let text = b"abcdefgh".to_vec();
        let mut ifb = IfStageBuilder::new(0, text.to_vec());
        let mut consts = ConstsBuilder::default();
        consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(1);
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
        assert_eq!(imem.read(), 0);
        if_.rasing_edge();
        assert_eq!(pc.read(), 0);
        assert_eq!(npc.read(), 4);
        assert_eq!(imem.read(), 0);
        if_.falling_edge();
        assert_eq!(pc.read(), 4);
        assert_eq!(npc.read(), 8);
        assert_eq!(imem.read(), u32::from_ne_bytes([b'a', b'b', b'c', b'd']));
    }
}
