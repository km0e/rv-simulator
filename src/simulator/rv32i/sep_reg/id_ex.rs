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
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::RegWrite => self.reg_write.alloc(RegAlloc::Out),
            Alloc::WbSel => self.wb_sel.alloc(RegAlloc::Out),
            Alloc::MemWrite => self.mem_write.alloc(RegAlloc::Out),
            Alloc::Jal_ => self.jal_.alloc(RegAlloc::Out),
            Alloc::BranchEn => self.branch_sel.alloc(RegAlloc::Out),
            Alloc::PcSel => self.pc_sel.alloc(RegAlloc::Out),
            Alloc::ImmSel => self.imm_sel.alloc(RegAlloc::Out),
            Alloc::AluCtrl => self.alu_ctrl.alloc(RegAlloc::Out),
            Alloc::BranchType => self.branch_type.alloc(RegAlloc::Out),
            Alloc::Npc => self.npc.alloc(RegAlloc::Out),
            Alloc::Pc => self.pc.alloc(RegAlloc::Out),
            Alloc::Rs1Data => self.rs1_data.alloc(RegAlloc::Out),
            Alloc::Rs2Data => self.rs2_data.alloc(RegAlloc::Out),
            Alloc::Imm => self.imm.alloc(RegAlloc::Out),
            Alloc::Rs1 => self.rs1.alloc(RegAlloc::Out),
            Alloc::Rd => self.rd.alloc(RegAlloc::Out),
            Alloc::Rs2 => self.rs2.alloc(RegAlloc::Out),
            Alloc::Opco => self.opco.alloc(RegAlloc::Out),
            Alloc::LoadSignal => self.load_signal.alloc(RegAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::RegWrite => self.reg_write.connect(pin, RegConnect::In),
            Connect::WbSel => self.wb_sel.connect(pin, RegConnect::In),
            Connect::MemWrite => self.mem_write.connect(pin, RegConnect::In),
            Connect::Jal_ => self.jal_.connect(pin, RegConnect::In),
            Connect::BranchEn => self.branch_sel.connect(pin, RegConnect::In),
            Connect::PcSel => self.pc_sel.connect(pin, RegConnect::In),
            Connect::ImmSel => self.imm_sel.connect(pin, RegConnect::In),
            Connect::AluCtrl => self.alu_ctrl.connect(pin, RegConnect::In),
            Connect::BranchType => self.branch_type.connect(pin, RegConnect::In),
            Connect::Npc => self.npc.connect(pin, RegConnect::In),
            Connect::Pc => self.pc.connect(pin, RegConnect::In),
            Connect::Rs1Data => self.rs1_data.connect(pin, RegConnect::In),
            Connect::Rs2Data => self.rs2_data.connect(pin, RegConnect::In),
            Connect::Imm => self.imm.connect(pin, RegConnect::In),
            Connect::Rs1 => self.rs1.connect(pin, RegConnect::In),
            Connect::Rd => self.rd.connect(pin, RegConnect::In),
            Connect::Rs2 => self.rs2.connect(pin, RegConnect::In),
            Connect::Opcode => self.opco.connect(pin, RegConnect::In),
            Connect::Enable => {
                self.reg_write.connect(pin.clone(), RegConnect::Enable);
                self.wb_sel.connect(pin.clone(), RegConnect::Enable);
                self.mem_write.connect(pin.clone(), RegConnect::Enable);
                self.jal_.connect(pin.clone(), RegConnect::Enable);
                self.branch_sel.connect(pin.clone(), RegConnect::Enable);
                self.pc_sel.connect(pin.clone(), RegConnect::Enable);
                self.imm_sel.connect(pin.clone(), RegConnect::Enable);
                self.alu_ctrl.connect(pin.clone(), RegConnect::Enable);
                self.branch_type.connect(pin.clone(), RegConnect::Enable);
                self.npc.connect(pin.clone(), RegConnect::Enable);
                self.pc.connect(pin.clone(), RegConnect::Enable);
                self.rs1_data.connect(pin.clone(), RegConnect::Enable);
                self.rs2_data.connect(pin.clone(), RegConnect::Enable);
                self.imm.connect(pin.clone(), RegConnect::Enable);
                self.rs1.connect(pin.clone(), RegConnect::Enable);
                self.rd.connect(pin.clone(), RegConnect::Enable);
                self.rs2.connect(pin.clone(), RegConnect::Enable);
                self.opco.connect(pin.clone(), RegConnect::Enable);
                self.load_signal.connect(pin, RegConnect::Enable);
            }
            Connect::Clear => {
                self.reg_write.connect(pin.clone(), RegConnect::Clear);
                self.wb_sel.connect(pin.clone(), RegConnect::Clear);
                self.mem_write.connect(pin.clone(), RegConnect::Clear);
                self.jal_.connect(pin.clone(), RegConnect::Clear);
                self.branch_sel.connect(pin.clone(), RegConnect::Clear);
                self.pc_sel.connect(pin.clone(), RegConnect::Clear);
                self.imm_sel.connect(pin.clone(), RegConnect::Clear);
                self.alu_ctrl.connect(pin.clone(), RegConnect::Clear);
                self.branch_type.connect(pin.clone(), RegConnect::Clear);
                self.npc.connect(pin.clone(), RegConnect::Clear);
                self.pc.connect(pin.clone(), RegConnect::Clear);
                self.rs1_data.connect(pin.clone(), RegConnect::Clear);
                self.rs2_data.connect(pin.clone(), RegConnect::Clear);
                self.imm.connect(pin.clone(), RegConnect::Clear);
                self.rs1.connect(pin.clone(), RegConnect::Clear);
                self.rd.connect(pin.clone(), RegConnect::Clear);
                self.rs2.connect(pin.clone(), RegConnect::Clear);
                self.opco.connect(pin.clone(), RegConnect::Clear);
                self.load_signal.connect(pin, RegConnect::Clear);
            }
            Connect::LoadSignal => self.load_signal.connect(pin, RegConnect::In),
        }
    }
}
#[derive(Debug)]
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
    fn input(&self) -> Vec<(String, u32)> {
        unimplemented!()
    }
    fn output(&self) -> Vec<(String, u32)> {
        unimplemented!()
    }
    fn inout(&self) -> Vec<(String, u32, u32)> {
        let mut res = Vec::new();
        res.push((
            "reg_write".to_string(),
            self.reg_write.input()[0].1,
            self.reg_write.output()[0].1,
        ));
        res.push((
            "wb_sel".to_string(),
            self.wb_sel.input()[0].1,
            self.wb_sel.output()[0].1,
        ));
        res.push((
            "mem_write".to_string(),
            self.mem_write.input()[0].1,
            self.mem_write.output()[0].1,
        ));
        res.push((
            "jal_".to_string(),
            self.jal_.input()[0].1,
            self.jal_.output()[0].1,
        ));
        res.push((
            "branch_sel".to_string(),
            self.branch_sel.input()[0].1,
            self.branch_sel.output()[0].1,
        ));
        res.push((
            "pc_sel".to_string(),
            self.pc_sel.input()[0].1,
            self.pc_sel.output()[0].1,
        ));
        res.push((
            "imm_sel".to_string(),
            self.imm_sel.input()[0].1,
            self.imm_sel.output()[0].1,
        ));
        res.push((
            "alu_ctrl".to_string(),
            self.alu_ctrl.input()[0].1,
            self.alu_ctrl.output()[0].1,
        ));
        res.push((
            "branch_type".to_string(),
            self.branch_type.input()[0].1,
            self.branch_type.output()[0].1,
        ));
        res.push((
            "npc".to_string(),
            self.npc.input()[0].1,
            self.npc.output()[0].1,
        ));
        res.push((
            "pc".to_string(),
            self.pc.input()[0].1,
            self.pc.output()[0].1,
        ));
        res.push((
            "rs1_data".to_string(),
            self.rs1_data.input()[0].1,
            self.rs1_data.output()[0].1,
        ));
        res.push((
            "rs2_data".to_string(),
            self.rs2_data.input()[0].1,
            self.rs2_data.output()[0].1,
        ));
        res.push((
            "imm".to_string(),
            self.imm.input()[0].1,
            self.imm.output()[0].1,
        ));
        res.push((
            "rs1".to_string(),
            self.rs1.input()[0].1,
            self.rs1.output()[0].1,
        ));
        res.push((
            "rd".to_string(),
            self.rd.input()[0].1,
            self.rd.output()[0].1,
        ));
        res.push((
            "rs2".to_string(),
            self.rs2.input()[0].1,
            self.rs2.output()[0].1,
        ));
        res.push((
            "opco".to_string(),
            self.opco.input()[0].1,
            self.opco.output()[0].1,
        ));
        res.push((
            "load_signal".to_string(),
            self.load_signal.input()[0].1,
            self.load_signal.output()[0].1,
        ));
        res
    }
}

pub mod build {
    pub use super::Alloc as IdExAlloc;
    pub use super::Connect as IdExConnect;
    pub use super::IdExBuilder;
}
