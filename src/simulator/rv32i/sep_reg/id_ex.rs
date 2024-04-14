use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    RegWrite = 0,
    WbSel = 1,
    MemWrite = 2,
    //
    Jal_ = 4,
    BranchEn = 5,
    PcSel = 6,
    ImmSel = 7,
    AluCtrl = 8,
    BranchType = 9,
    Npc = 10,
    Pc = 11,
    Rs1Data = 12,
    Rs2Data = 13,
    Imm = 14,
    Rs1 = 15,
    Rd = 16,
    Rs2 = 17,
    Opco = 18,
    LoadSignal = 19,
}
impl From<Alloc> for usize {
    fn from(id: Alloc) -> Self {
        match id {
            Alloc::RegWrite => 0,
            Alloc::WbSel => 1,
            Alloc::MemWrite => 2,
            Alloc::Jal_ => 4,
            Alloc::BranchEn => 5,
            Alloc::PcSel => 6,
            Alloc::ImmSel => 7,
            Alloc::AluCtrl => 8,
            Alloc::BranchType => 9,
            Alloc::Npc => 10,
            Alloc::Pc => 11,
            Alloc::Rs1Data => 12,
            Alloc::Rs2Data => 13,
            Alloc::Imm => 14,
            Alloc::Rs1 => 15,
            Alloc::Rd => 16,
            Alloc::Rs2 => 17,
            Alloc::Opco => 18,
            Alloc::LoadSignal => 18,
        }
    }
}

pub enum Connect {
    RegWrite = 0,
    WbSel = 1,
    MemWrite = 2,
    //
    Jal_ = 4,
    BranchEn = 5,
    PcSel = 6,
    ImmSel = 7,
    AluCtrl = 8,
    BranchType = 9,
    Npc = 10,
    Pc = 11,
    Rs1Data = 12,
    Rs2Data = 13,
    Imm = 14,
    Rs1 = 15,
    Rd = 16,
    Rs2 = 17,
    Opcode = 18,
    Enable = 19,
    Clear = 20,
    LoadSignal = 21,
}
impl From<Connect> for usize {
    fn from(id: Connect) -> Self {
        match id {
            Connect::RegWrite => 0,
            Connect::WbSel => 1,
            Connect::MemWrite => 2,
            Connect::Jal_ => 4,
            Connect::BranchEn => 5,
            Connect::PcSel => 6,
            Connect::ImmSel => 7,
            Connect::AluCtrl => 8,
            Connect::BranchType => 9,
            Connect::Npc => 10,
            Connect::Pc => 11,
            Connect::Rs1Data => 12,
            Connect::Rs2Data => 13,
            Connect::Imm => 14,
            Connect::Rs1 => 15,
            Connect::Rd => 16,
            Connect::Rs2 => 17,
            Connect::Opcode => 18,
            Connect::Enable => 19,
            Connect::Clear => 20,
            Connect::LoadSignal => 21,
        }
    }
}

#[derive(Default)]
pub struct IdExBuilder {
    pub reg_write: RegBuilder,
    pub wb_sel: RegBuilder,
    pub mem_write: RegBuilder,
    pub jal_: RegBuilder,
    pub branch_sel: RegBuilder,
    pub pc_sel: RegBuilder,
    pub imm_sel: RegBuilder,
    pub alu_ctrl: RegBuilder,
    pub branch_type: RegBuilder,
    pub npc: RegBuilder,
    pub pc: RegBuilder,
    pub rs1_data: RegBuilder,
    pub rs2_data: RegBuilder,
    pub imm: RegBuilder,
    pub rs1: RegBuilder,
    pub rd: RegBuilder,
    pub rs2: RegBuilder,
    pub opco: RegBuilder,
    pub load_signal: RegBuilder,
    pub asm: AsmRegBuilder,
}
impl AsmBuilder for IdExBuilder {
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
        self.asm.asm_connect(pin, id);
    }
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        self.asm.asm_alloc(id)
    }
}
impl ControlBuilder for IdExBuilder {
    fn build(self) -> ControlRef {
        IdEx {
            reg_write: self.reg_write.build(),
            wb_sel: self.wb_sel.build(),
            mem_write: self.mem_write.build(),
            jal_: self.jal_.build(),
            branch_sel: self.branch_sel.build(),
            pc_sel: self.pc_sel.build(),
            imm_sel: self.imm_sel.build(),
            alu_ctrl: self.alu_ctrl.build(),
            branch_type: self.branch_type.build(),
            npc: self.npc.build(),
            pc: self.pc.build(),
            rs1_data: self.rs1_data.build(),
            rs2_data: self.rs2_data.build(),
            imm: self.imm.build(),
            rs1: self.rs1.build(),
            rd: self.rd.build(),
            rs2: self.rs2.build(),
            opco: self.opco.build(),
            load_signal: self.load_signal.build(),
            asm: self.asm.build(),
        }
        .into()
    }
}
impl PortBuilder for IdExBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => self.reg_write.alloc(RegAlloc::Out.into()),
            1 => self.wb_sel.alloc(RegAlloc::Out.into()),
            2 => self.mem_write.alloc(RegAlloc::Out.into()),
            4 => self.jal_.alloc(RegAlloc::Out.into()),
            5 => self.branch_sel.alloc(RegAlloc::Out.into()),
            6 => self.pc_sel.alloc(RegAlloc::Out.into()),
            7 => self.imm_sel.alloc(RegAlloc::Out.into()),
            8 => self.alu_ctrl.alloc(RegAlloc::Out.into()),
            9 => self.branch_type.alloc(RegAlloc::Out.into()),
            10 => self.npc.alloc(RegAlloc::Out.into()),
            11 => self.pc.alloc(RegAlloc::Out.into()),
            12 => self.rs1_data.alloc(RegAlloc::Out.into()),
            13 => self.rs2_data.alloc(RegAlloc::Out.into()),
            14 => self.imm.alloc(RegAlloc::Out.into()),
            15 => self.rs1.alloc(RegAlloc::Out.into()),
            16 => self.rd.alloc(RegAlloc::Out.into()),
            17 => self.rs2.alloc(RegAlloc::Out.into()),
            18 => self.opco.alloc(RegAlloc::Out.into()),
            19 => self.load_signal.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.reg_write.connect(pin, RegConnect::In.into()),
            1 => self.wb_sel.connect(pin, RegConnect::In.into()),
            2 => self.mem_write.connect(pin, RegConnect::In.into()),
            4 => self.jal_.connect(pin, RegConnect::In.into()),
            5 => self.branch_sel.connect(pin, RegConnect::In.into()),
            6 => self.pc_sel.connect(pin, RegConnect::In.into()),
            7 => self.imm_sel.connect(pin, RegConnect::In.into()),
            8 => self.alu_ctrl.connect(pin, RegConnect::In.into()),
            9 => self.branch_type.connect(pin, RegConnect::In.into()),
            10 => self.npc.connect(pin, RegConnect::In.into()),
            11 => self.pc.connect(pin, RegConnect::In.into()),
            12 => self.rs1_data.connect(pin, RegConnect::In.into()),
            13 => self.rs2_data.connect(pin, RegConnect::In.into()),
            14 => self.imm.connect(pin, RegConnect::In.into()),
            15 => self.rs1.connect(pin, RegConnect::In.into()),
            16 => self.rd.connect(pin, RegConnect::In.into()),
            17 => self.rs2.connect(pin, RegConnect::In.into()),
            18 => self.opco.connect(pin, RegConnect::In.into()),
            19 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.mem_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.jal_.connect(pin.clone(), RegConnect::Enable.into());
                self.branch_sel
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.pc_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.imm_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.alu_ctrl
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.branch_type
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.npc.connect(pin.clone(), RegConnect::Enable.into());
                self.pc.connect(pin.clone(), RegConnect::Enable.into());
                self.rs1_data
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.rs2_data
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.imm.connect(pin.clone(), RegConnect::Enable.into());
                self.rs1.connect(pin.clone(), RegConnect::Enable.into());
                self.rd.connect(pin.clone(), RegConnect::Enable.into());
                self.rs2.connect(pin.clone(), RegConnect::Enable.into());
                self.opco.connect(pin.clone(), RegConnect::Enable.into());
                self.load_signal.connect(pin, RegConnect::Enable.into());
            }
            20 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.mem_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.jal_.connect(pin.clone(), RegConnect::Clear.into());
                self.branch_sel
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.pc_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.imm_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.alu_ctrl.connect(pin.clone(), RegConnect::Clear.into());
                self.branch_type
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.npc.connect(pin.clone(), RegConnect::Clear.into());
                self.pc.connect(pin.clone(), RegConnect::Clear.into());
                self.rs1_data.connect(pin.clone(), RegConnect::Clear.into());
                self.rs2_data.connect(pin.clone(), RegConnect::Clear.into());
                self.imm.connect(pin.clone(), RegConnect::Clear.into());
                self.rs1.connect(pin.clone(), RegConnect::Clear.into());
                self.rd.connect(pin.clone(), RegConnect::Clear.into());
                self.rs2.connect(pin.clone(), RegConnect::Clear.into());
                self.opco.connect(pin.clone(), RegConnect::Clear.into());
                self.load_signal.connect(pin, RegConnect::Clear.into());
            }
            21 => self.load_signal.connect(pin, RegConnect::In.into()),
            _ => panic!("Invalid id"),
        }
    }
}
pub struct IdEx {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub mem_write: ControlRef,
    pub jal_: ControlRef,
    pub branch_sel: ControlRef,
    pub pc_sel: ControlRef,
    pub imm_sel: ControlRef,
    pub alu_ctrl: ControlRef,
    pub branch_type: ControlRef,
    pub npc: ControlRef,
    pub pc: ControlRef,
    pub rs1_data: ControlRef,
    pub rs2_data: ControlRef,
    pub imm: ControlRef,
    pub rs1: ControlRef,
    pub rd: ControlRef,
    pub rs2: ControlRef,
    pub opco: ControlRef,
    pub load_signal: ControlRef,
    pub asm: ControlRef,
}

impl Control for IdEx {
    fn rasing_edge(&mut self) {
        self.reg_write.rasing_edge();
        self.wb_sel.rasing_edge();
        self.mem_write.rasing_edge();
        self.jal_.rasing_edge();
        self.branch_sel.rasing_edge();
        self.pc_sel.rasing_edge();
        self.imm_sel.rasing_edge();
        self.alu_ctrl.rasing_edge();
        self.branch_type.rasing_edge();
        self.npc.rasing_edge();
        self.pc.rasing_edge();
        self.rs1_data.rasing_edge();
        self.rs2_data.rasing_edge();
        self.imm.rasing_edge();
        self.rs1.rasing_edge();
        self.rd.rasing_edge();
        self.rs2.rasing_edge();
        self.opco.rasing_edge();
        self.load_signal.rasing_edge();
        self.asm.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.reg_write.falling_edge();
        self.wb_sel.falling_edge();
        self.mem_write.falling_edge();
        self.jal_.falling_edge();
        self.branch_sel.falling_edge();
        self.pc_sel.falling_edge();
        self.imm_sel.falling_edge();
        self.alu_ctrl.falling_edge();
        self.branch_type.falling_edge();
        self.npc.falling_edge();
        self.pc.falling_edge();
        self.rs1_data.falling_edge();
        self.rs2_data.falling_edge();
        self.imm.falling_edge();
        self.rs1.falling_edge();
        self.rd.falling_edge();
        self.rs2.falling_edge();
        self.opco.falling_edge();
        self.load_signal.falling_edge();
        self.asm.falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "ID/EX : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} MEM_WRITE\t: {:8} JAL_\t\t: {:8} BRANCH_EN\t: {:8}\nPC_SEL\t\t: {:8} IMM_SEL\t: {:8} ALU_CTRL\t: {:8} BRANCH_TYPE\t: {:8} NPC\t\t: {:8}\nPC\t\t: {:8} RS1_DATA\t: {:8} RS2_DATA\t: {:8} IMM\t\t: {:8} RS1\t\t: {:8}\nRD\t\t: {:8} RS2\t\t: {:8} OPCODE\t: {:8} LOAD_SIGNAL\t: {}",
            self.asm.debug(),
            self.reg_write.debug(),
            self.wb_sel.debug(),
            self.mem_write.debug(),
            self.jal_.debug(),
            self.branch_sel.debug(),
            self.pc_sel.debug(),
            self.imm_sel.debug(),
            self.alu_ctrl.debug(),
            self.branch_type.debug(),
            self.npc.debug(),
            self.pc.debug(),
            self.rs1_data.debug(),
            self.rs2_data.debug(),
            self.imm.debug(),
            self.rs1.debug(),
            self.rd.debug(),
            self.rs2.debug(),
            self.opco.debug(),
            self.load_signal.debug()
        )
    }
}

pub mod build {
    pub use super::Alloc as IdExAlloc;
    pub use super::Connect as IdExConnect;
    pub use super::IdExBuilder;
}
