use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Npc,
    Pc,
    Instruction,
}
pub enum Connect {
    Npc = 0,
    Pc = 1,
    Instruction = 2,
    Enable = 3,
    Clear = 4,
}

#[derive(Default)]
pub struct IfIdBuilder {
    pub npc: RegBuilder,
    pub pc: RegBuilder,
    pub instruction: RegBuilder,
}
impl ControlBuilder for IfIdBuilder {
    fn build(self) -> ControlRef {
        IfId {
            npc: self.npc.build(),
            pc: self.pc.build(),
            instruction: self.instruction.build(),
        }
        .into()
    }
}
impl PortBuilder for IfIdBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::Npc => self.npc.alloc(RegAlloc::Out),
            Alloc::Pc => self.pc.alloc(RegAlloc::Out),
            Alloc::Instruction => self.instruction.alloc(RegAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Npc => self.npc.connect(pin, RegConnect::In),
            Connect::Pc => self.pc.connect(pin, RegConnect::In),
            Connect::Instruction => self.instruction.connect(pin, RegConnect::In),
            Connect::Enable => {
                self.npc.connect(pin.clone(), RegConnect::Enable);
                self.pc.connect(pin.clone(), RegConnect::Enable);
                self.instruction.connect(pin, RegConnect::Enable);
            }
            Connect::Clear => {
                self.npc.connect(pin.clone(), RegConnect::Clear);
                self.pc.connect(pin.clone(), RegConnect::Clear);
                self.instruction.connect(pin, RegConnect::Clear);
            }
        }
    }
}

#[derive(Debug)]
pub struct IfId {
    pub npc: ControlRef,
    pub pc: ControlRef,
    pub instruction: ControlRef,
}
impl Control for IfId {
    fn rasing_edge(&mut self) {
        self.npc.rasing_edge();
        self.pc.rasing_edge();
        self.instruction.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.npc.falling_edge();
        self.pc.falling_edge();
        self.instruction.falling_edge();
    }
    fn inout(&self) -> Vec<(&'static str, u32, u32)> {
        vec![
            ("npc", self.npc.input()[0].1, self.npc.output()[0].1),
            ("pc", self.pc.input()[0].1, self.pc.output()[0].1),
            (
                "instruction",
                self.instruction.input()[0].1,
                self.instruction.output()[0].1,
            ),
        ]
    }
}
pub mod build {
    pub use super::Alloc as IfIdAlloc;
    pub use super::Connect as IfIdConnect;
    pub use super::IfIdBuilder;
}
