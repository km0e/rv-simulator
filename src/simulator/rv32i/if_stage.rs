use crate::circuit::Circuit;
use crate::component::{build::*, Control, ControlRef, ControlShared};
pub struct IfBuilder {
    pub npc_mux: MuxBuilder,
    pub pc: RegBuilder,
    pub add: AddBuilder,
    pub imem: MemBuilder,
}
impl IfBuilder {
    pub fn new(instruction_memory: Vec<u8>, entry: u32) -> Self {
        // add if stage
        // set up consts
        let mut consts = ConstsBuilder::default();
        consts.push(4);
        consts.push(2);
        consts.push(0);
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
        if_npc_mux.connect(if_add.alloc(0), 1);
        // set up instruction memory
        let mut if_imem = MemBuilder::new(instruction_memory);
        if_imem.connect(if_pc.alloc(0), 0);
        // build if stage
        IfBuilder {
            npc_mux: if_npc_mux,
            pc: if_pc,
            add: if_add,
            imem: if_imem,
        }
    }
}
impl Builder for IfBuilder {
    fn alloc(&mut self, id: usize) -> ComponentRef {
        match id {
            0 => self.add.alloc(0),
            1 => self.pc.alloc(0),
            2 => self.imem.alloc(0),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        match id {
            0 => self.npc_mux.connect(pin, 0),
            1 => self.npc_mux.connect(pin, 2),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<ControlRef> {
        self.npc_mux.build();
        self.add.build();
        let mut circuit = Circuit::default();
        circuit.add(self.pc.build().unwrap());
        circuit.add(self.imem.build().unwrap());
        Some(ControlRef::from(ControlShared::new(If { circuit })))
    }
}
pub struct If {
    pub circuit: Circuit,
}
impl Control for If {
    fn rasing_edge(&mut self) {
        self.circuit.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.circuit.falling_edge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_if() {
        let text = b"abcdefgh".to_vec();
        let mut ifb = IfBuilder::new(text.to_vec(), 0);
        let mut consts = ConstsBuilder::default();
        consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(0);
        ifb.npc_mux.connect(consts.alloc(0), 0);
        ifb.imem.connect(consts.alloc(1), 2);
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
