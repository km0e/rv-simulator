use std::cell::RefCell;
use std::rc::Rc;

use crate::circuit::Circuit;
use crate::component::{build::*, Control, ControlRef, ControlShared};
use crate::simulator::asm::{Asm, AsmBuilder, AsmReg, Connect as AsmConnect};
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
    pub pc_cache: RegBuilder,
    pub add: AddBuilder,
    pub npc_cache: RegBuilder,
    pub imem: MemBuilder,
    pub asm: AsmBuilder,
}
impl IfStageBuilder {
    pub fn new(entry: u32, instruction_memory: Vec<u8>, asm_mem: Vec<String>) -> Self {
        // add if stage
        // set up consts
        let mut consts = ConstsBuilder::default();
        consts.push(4);
        consts.push(2);
        consts.push(0);
        consts.push(1);
        // set up pc increment mux
        let mut if_pc_inc = MuxBuilder::default();
        if_pc_inc.connect(consts.alloc(0), 1);
        if_pc_inc.connect(consts.alloc(1), 2);
        if_pc_inc.connect(consts.alloc(2), 0);
        // set up npc mux
        let mut if_npc_mux = MuxBuilder::default();
        // set up pc
        let mut if_pc = RegBuilder::new(entry);
        if_pc.connect(if_npc_mux.alloc(0), 0);
        // set up add
        let mut if_add = AddBuilder::new();
        if_add.connect(if_pc_inc.alloc(0), 0);
        if_pc_inc.build();
        if_add.connect(if_pc.alloc(0), 1);
        // connect npc and add
        if_npc_mux.connect(if_add.alloc(0), MuxConnect::In(0).into());
        // set up instruction memory
        let mut if_imem = MemBuilder::new(instruction_memory);
        if_imem.connect(if_pc.alloc(0), MemConnect::Address.into());
        if_imem.connect(consts.alloc(2), MemConnect::Write.into());
        if_imem.connect(consts.alloc(3), MemConnect::Read.into());
        //cache
        let mut if_npc_cache = RegBuilder::new(0);
        if_npc_cache.connect(if_add.alloc(0), RegConnect::In.into());
        if_npc_cache.connect(consts.alloc(3), RegConnect::Enable.into());
        if_npc_cache.connect(consts.alloc(2), RegConnect::Clear.into());
        let mut if_pc_cache = RegBuilder::new(0);
        if_pc_cache.connect(if_pc.alloc(RegAlloc::Out.into()), RegConnect::In.into());
        if_pc_cache.connect(consts.alloc(3), RegConnect::Enable.into());
        if_pc_cache.connect(consts.alloc(2), RegConnect::Clear.into());
        //asm
        let mut if_asm = AsmBuilder::new(asm_mem);
        if_asm.connect(
            if_pc.alloc(RegAlloc::Out.into()),
            AsmConnect::Address.into(),
        );
        // build if stage
        IfStageBuilder {
            npc_mux: if_npc_mux,
            pc: if_pc,
            add: if_add,
            imem: if_imem,
            pc_cache: if_pc_cache,
            npc_cache: if_npc_cache,
            asm: if_asm,
        }
    }
    pub fn alloc_asm(&mut self) -> Rc<RefCell<AsmReg>> {
        self.asm.alloc_asm()
    }
}
impl Builder for IfStageBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            // 0 => self.pc.alloc(0),
            // 1 => self.add.alloc(0),
            0 => self.pc_cache.alloc(0),
            1 => self.npc_cache.alloc(0),
            2 => self.imem.alloc(0),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.pc.connect(pin, RegConnect::Enable.into()),
            1 => self.npc_mux.connect(pin, MuxConnect::Select.into()),
            2 => self.npc_mux.connect(pin, MuxConnect::In(1).into()),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<ControlRef> {
        self.npc_mux.build();
        self.add.build();
        Some(ControlRef::from(ControlShared::new(IfStage {
            pc: self.pc.build().unwrap(),
            imem: self.imem.build().unwrap(),
            pc_cache: self.pc_cache.build().unwrap(),
            npc_cache: self.npc_cache.build().unwrap(),
            asm: self.asm.inner.into(),
        })))
    }
}
pub struct IfStage {
    pub pc: ControlRef,
    pub imem: ControlRef,
    pub pc_cache: ControlRef,
    pub npc_cache: ControlRef,
    pub asm: ControlRef,
}
impl Control for IfStage {
    fn rasing_edge(&mut self) {
        self.pc.rasing_edge();
        self.pc_cache.rasing_edge();
        self.npc_cache.rasing_edge();
        self.imem.rasing_edge();
        self.asm.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.pc.falling_edge();
        self.pc_cache.falling_edge();
        self.npc_cache.falling_edge();
        self.imem.falling_edge();
        self.asm.falling_edge();
    }
    fn debug(&self) -> String {
        format!(
            "IfStage\npc: {}\nimem: {}",
            self.pc.debug(),
            self.imem.debug()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_if() {
        let text = b"abcdefgh".to_vec();
        let mut ifb = IfStageBuilder::new(0, text.to_vec(), vec![]);
        let mut consts = ConstsBuilder::default();
        consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(1);
        ifb.npc_mux.connect(consts.alloc(0), 0);
        ifb.imem.connect(consts.alloc(0), 2);
        ifb.connect(consts.alloc(1), Connect::PcEnable.into());
        let pc = ifb.pc.alloc(0);
        let npc = ifb.add.alloc(0);
        let imem = ifb.imem.alloc(0);
        let if_ = ifb.build().unwrap();
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
