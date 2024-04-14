use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Npc,
    Pc,
    Instruction,
}
impl From<Alloc> for usize {
    fn from(id: Alloc) -> Self {
        match id {
            Alloc::Npc => 0,
            Alloc::Pc => 1,
            Alloc::Instruction => 2,
        }
    }
}

pub enum Connect {
    Npc = 0,
    Pc = 1,
    Instruction = 2,
    Enable = 3,
    Clear = 4,
}

impl From<Connect> for usize {
    fn from(id: Connect) -> Self {
        match id {
            Connect::Npc => 0,
            Connect::Pc => 1,
            Connect::Instruction => 2,
            Connect::Enable => 3,
            Connect::Clear => 4,
        }
    }
}

#[derive(Default)]
pub struct IfIdBuilder {
    pub npc: RegBuilder,
    pub pc: RegBuilder,
    pub instruction: RegBuilder,
    pub asm: AsmRegBuilder,
}
impl AsmBuilder for IfIdBuilder {
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
        self.asm.asm_connect(pin, id);
    }
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        self.asm.asm_alloc(id)
    }
}
impl ControlBuilder for IfIdBuilder {
    fn build(self) -> ControlRef {
        IfId {
            npc: self.npc.build(),
            pc: self.pc.build(),
            instruction: self.instruction.build(),
            asm: self.asm.build(),
        }
        .into()
    }
}
impl PortBuilder for IfIdBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => self.npc.alloc(RegAlloc::Out.into()),
            1 => self.pc.alloc(RegAlloc::Out.into()),
            2 => self.instruction.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.npc.connect(pin, RegConnect::In.into()),
            1 => self.pc.connect(pin, RegConnect::In.into()),
            2 => self.instruction.connect(pin, RegConnect::In.into()),
            3 => {
                self.npc.connect(pin.clone(), RegConnect::Enable.into());
                self.pc.connect(pin.clone(), RegConnect::Enable.into());
                self.instruction.connect(pin, RegConnect::Enable.into());
            }
            4 => {
                self.npc.connect(pin.clone(), RegConnect::Clear.into());
                self.pc.connect(pin.clone(), RegConnect::Clear.into());
                self.instruction.connect(pin, RegConnect::Clear.into());
            }
            _ => panic!("Invalid id"),
        }
    }
}

pub struct IfId {
    pub npc: ControlRef,
    pub pc: ControlRef,
    pub instruction: ControlRef,
    pub asm: ControlRef,
}
impl Control for IfId {
    fn rasing_edge(&mut self) {
        self.npc.rasing_edge();
        self.pc.rasing_edge();
        self.instruction.rasing_edge();
        self.asm.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.npc.falling_edge();
        self.pc.falling_edge();
        self.instruction.falling_edge();
        self.asm.falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "IF/ID : {}\nNPC\t\t: {:8} PC\t\t: {:8} INST\t\t: {:8}",
            self.asm.debug(),
            self.npc.debug(),
            self.pc.debug(),
            self.instruction.debug()
        )
    }
}
pub mod build {
    pub use super::Alloc as IfIdAlloc;
    pub use super::Connect as IfIdConnect;
    pub use super::IfIdBuilder;
}
