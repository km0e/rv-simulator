use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    RegWrite = 0,
    WbSel = 1,
    MemWrite = 2,
    //
    Npc = 4,
    AluRes = 5,
    Rs2Data = 6,
    Rd = 7,
    MemRead = 8,
}

impl From<Alloc> for usize {
    fn from(id: Alloc) -> Self {
        match id {
            Alloc::RegWrite => 0,
            Alloc::WbSel => 1,
            Alloc::MemWrite => 2,
            Alloc::Npc => 4,
            Alloc::AluRes => 5,
            Alloc::Rs2Data => 6,
            Alloc::Rd => 7,
            Alloc::MemRead => 8,
        }
    }
}

pub enum Connect {
    RegWrite = 0,
    WbSel = 1,
    MemWrite = 2,
    //
    Npc = 4,
    AluRes = 5,
    Rs2Data = 6,
    Rd = 7,
    Ebable = 8,
    Clear = 9,
    MemRead = 10,
}

impl From<Connect> for usize {
    fn from(id: Connect) -> Self {
        match id {
            Connect::RegWrite => 0,
            Connect::WbSel => 1,
            Connect::MemWrite => 2,
            Connect::Npc => 4,
            Connect::AluRes => 5,
            Connect::Rs2Data => 6,
            Connect::Rd => 7,
            Connect::Ebable => 8,
            Connect::Clear => 9,
            Connect::MemRead => 10,
        }
    }
}

#[derive(Default)]
pub struct ExMemBuilder {
    pub reg_write: RegBuilder,
    pub wb_sel: RegBuilder,
    pub mem_write: RegBuilder,
    pub npc: RegBuilder,
    pub alu_res: RegBuilder,
    pub rs2_data: RegBuilder,
    pub rd: RegBuilder,
    pub mem_read: RegBuilder,
    pub asm: AsmRegBuilder,
}
impl AsmBuilder for ExMemBuilder {
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
        self.asm.asm_connect(pin, id);
    }
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        self.asm.asm_alloc(id)
    }
}
impl ControlBuilder for ExMemBuilder {
    fn build(self) -> ControlRef {
        Some(ControlRef::from(ControlShared::new(ExMem {
            reg_write: self.reg_write.build().unwrap(),
            wb_sel: self.wb_sel.build().unwrap(),
            mem_write: self.mem_write.build().unwrap(),
            npc: self.npc.build().unwrap(),
            alu_res: self.alu_res.build().unwrap(),
            rs2_data: self.rs2_data.build().unwrap(),
            rd: self.rd.build().unwrap(),
            mem_read: self.mem_read.build().unwrap(),
            asm: self.asm.build(),
        })))
    }
}
impl PortBuilder for ExMemBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => self.reg_write.alloc(RegAlloc::Out.into()),
            1 => self.wb_sel.alloc(RegAlloc::Out.into()),
            2 => self.mem_write.alloc(RegAlloc::Out.into()),
            4 => self.npc.alloc(RegAlloc::Out.into()),
            5 => self.alu_res.alloc(RegAlloc::Out.into()),
            6 => self.rs2_data.alloc(RegAlloc::Out.into()),
            7 => self.rd.alloc(RegAlloc::Out.into()),
            8 => self.mem_read.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.reg_write.connect(pin, RegConnect::In.into()),
            1 => self.wb_sel.connect(pin, RegConnect::In.into()),
            2 => self.mem_write.connect(pin, RegConnect::In.into()),
            4 => self.npc.connect(pin, RegConnect::In.into()),
            5 => self.alu_res.connect(pin, RegConnect::In.into()),
            6 => self.rs2_data.connect(pin, RegConnect::In.into()),
            7 => self.rd.connect(pin, RegConnect::In.into()),
            8 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.mem_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.npc.connect(pin.clone(), RegConnect::Enable.into());
                self.alu_res.connect(pin.clone(), RegConnect::Enable.into());
                self.rs2_data
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.rd.connect(pin.clone(), RegConnect::Enable.into());
                self.mem_read.connect(pin, RegConnect::Enable.into());
            }
            9 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.mem_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.npc.connect(pin.clone(), RegConnect::Clear.into());
                self.alu_res.connect(pin.clone(), RegConnect::Clear.into());
                self.rs2_data.connect(pin.clone(), RegConnect::Clear.into());
                self.rd.connect(pin.clone(), RegConnect::Clear.into());
                self.mem_read.connect(pin, RegConnect::Clear.into());
            }
            10 => self.mem_read.connect(pin, RegConnect::In.into()),
            _ => panic!("Invalid id"),
        }
    }
}

pub struct ExMem {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub mem_write: ControlRef,
    pub mem_read: ControlRef,
    pub npc: ControlRef,
    pub alu_res: ControlRef,
    pub rs2_data: ControlRef,
    pub rd: ControlRef,
    pub asm: ControlRef,
}
impl Control for ExMem {
    fn rasing_edge(&mut self) {
        self.reg_write.rasing_edge();
        self.wb_sel.rasing_edge();
        self.mem_write.rasing_edge();
        self.npc.rasing_edge();
        self.alu_res.rasing_edge();
        self.rs2_data.rasing_edge();
        self.rd.rasing_edge();
        self.mem_read.rasing_edge();
        self.asm.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.reg_write.falling_edge();
        self.wb_sel.falling_edge();
        self.mem_write.falling_edge();
        self.npc.falling_edge();
        self.alu_res.falling_edge();
        self.rs2_data.falling_edge();
        self.rd.falling_edge();
        self.mem_read.falling_edge();
        self.asm.falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "EX/MEM : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} MEM_WRITE\t: {:8} NPC\t\t: {:8} ALU_RES\t: {:8}\nRS2_DATA\t: {:8} RD\t\t: {}",
            self.asm.debug(),
            self.reg_write.debug(),
            self.wb_sel.debug(),
            self.mem_write.debug(),
            self.npc.debug(),
            self.alu_res.debug(),
            self.rs2_data.debug(),
            self.rd.debug()
        )
    }
}

pub mod build {
    pub use super::Alloc as ExMemAlloc;
    pub use super::Connect as ExMemConnect;
    pub use super::ExMemBuilder;
}
