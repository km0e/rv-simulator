use std::cell::RefCell;
use std::rc::Rc;

use crate::component::{build::*, Builder, Control, ControlRef, ControlShared};

use super::asm::AsmReg;
use super::asm::AsmRegBuilder;
use super::utils;
mod ex_stage;
mod hazard;
mod id_stage;
mod if_stage;
mod mem_stage;
mod wb_stage;

use ex_stage::Alloc as ExAlloc;
use ex_stage::Connect as ExConnect;
use ex_stage::ExStageBuilder;
use hazard::Alloc as HazardAlloc;
use hazard::Connect as HazardConnect;
use hazard::HazardBuilder;
use id_stage::Alloc as IdAlloc;
use id_stage::Connect as IdConnect;
use id_stage::IdStageBuilder;
use if_stage::Alloc as IfAlloc;
use if_stage::Connect as IfConnect;
use if_stage::IfStageBuilder;
use mem_stage::Alloc as MemAlloc;
use mem_stage::Connect as MemConnect;
use mem_stage::MemStageBuilder;
use wb_stage::Alloc as WbAlloc;
use wb_stage::Connect as WbConnect;
use wb_stage::WbStageBuilder;
pub struct Rv32iBuilder {
    pub if_stage: IfStageBuilder,
    pub id_stage: IdStageBuilder,
    pub ex_stage: ExStageBuilder,
    pub mem_stage: MemStageBuilder,
    pub wb_stage: WbStageBuilder,
    pub hazard: HazardBuilder,
    pub if_id: IfIdBuilder,
    pub id_ex: IdExBuilder,
    pub ex_mem: ExMemBuilder,
    pub mem_wb: MemWbBuilder,
}

impl Rv32iBuilder {
    fn connect(inst_mem: Vec<u32>, asm_mem: Vec<String>) -> Self {
        let inst_mem = inst_mem.into_iter().flat_map(|x| x.to_ne_bytes()).collect();
        let mut if_stage = IfStageBuilder::new(0, inst_mem, asm_mem);
        let mut id_stage = IdStageBuilder::new(0x7FFFFFF0);
        let mut ex_stage = ExStageBuilder::new();
        let mut mem_stage = MemStageBuilder::default();
        let mut wb_stage = WbStageBuilder::default();
        let mut hazard = HazardBuilder::default();
        let mut if_id = IfIdBuilder::default();
        let mut ex_mem = ExMemBuilder::default();
        let mut mem_wb = MemWbBuilder::default();
        //set up if stage
        if_stage.connect(
            hazard.alloc(HazardAlloc::PcEnable.into()),
            IfConnect::PcEnable.into(),
        );
        if_stage.connect(
            ex_stage.alloc(ExAlloc::BranchSel.into()),
            IfConnect::NpcSel.into(),
        );
        if_stage.connect(
            ex_stage.alloc(ExAlloc::AluRes.into()),
            IfConnect::Npc.into(),
        );
        //set up if-id register
        if_id.connect(if_stage.alloc(IfAlloc::Npc.into()), IfIdConnect::Npc.into());
        if_id.connect(if_stage.alloc(IfAlloc::Pc.into()), IfIdConnect::Pc.into());
        if_id.connect(
            if_stage.alloc(IfAlloc::Imem.into()),
            IfIdConnect::Instruction.into(),
        );
        if_id.connect(
            hazard.alloc(HazardAlloc::IfIdEnable.into()),
            IfIdConnect::Enable.into(),
        );
        if_id.asm.inner.borrow_mut().prev = Some(if_stage.alloc_asm());
        //set up id stage
        id_stage.connect(
            if_id.alloc(IfIdAlloc::Instruction.into()),
            IdConnect::Inst.into(),
        );
        id_stage.connect(
            mem_wb.alloc(MemWbAlloc::RegWrite.into()),
            IdConnect::RegWrite.into(),
        );
        id_stage.connect(mem_wb.alloc(MemWbAlloc::Rd.into()), IdConnect::Rd.into());
        id_stage.connect(
            wb_stage.alloc(WbAlloc::Out.into()),
            IdConnect::RdData.into(),
        );
        //set up id-ex register
        let mut id_ex = IdExBuilder::default();
        id_ex.connect(
            id_stage.alloc(IdAlloc::RegWrite.into()),
            IdExConnect::RegWrite.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::WbSel.into()),
            IdExConnect::WbSel.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::MemWrite.into()),
            IdExConnect::MemWrite.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::Load.into()),
            IdExConnect::LoadSignal.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::Jal_.into()),
            IdExConnect::Jal_.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::BranchEn.into()),
            IdExConnect::BranchEn.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::PcSel.into()),
            IdExConnect::PcSel.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::ImmSel.into()),
            IdExConnect::ImmSel.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::AluCtrl.into()),
            IdExConnect::AluCtrl.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::BranchType.into()),
            IdExConnect::BranchType.into(),
        );
        id_ex.connect(if_id.alloc(IfIdAlloc::Npc.into()), IdExConnect::Npc.into());
        id_ex.connect(if_id.alloc(IfIdAlloc::Pc.into()), IdExConnect::Pc.into());
        id_ex.connect(
            id_stage.alloc(IdAlloc::Rs1Data.into()),
            IdExConnect::Rs1Data.into(),
        );
        id_ex.connect(
            id_stage.alloc(IdAlloc::Rs2Data.into()),
            IdExConnect::Rs2Data.into(),
        );
        id_ex.connect(id_stage.alloc(IdAlloc::Imm.into()), IdExConnect::Imm.into());
        id_ex.connect(id_stage.alloc(IdAlloc::Rs1.into()), IdExConnect::Rs1.into());
        id_ex.connect(id_stage.alloc(IdAlloc::Rd.into()), IdExConnect::Rd.into());
        id_ex.connect(id_stage.alloc(IdAlloc::Rs2.into()), IdExConnect::Rs2.into());
        id_ex.connect(
            id_stage.alloc(IdAlloc::Opcode.into()),
            IdExConnect::Opcode.into(),
        );
        id_ex.asm.inner.borrow_mut().prev = Some(if_id.alloc_asm());
        //set up ex stage
        ex_stage.connect(id_ex.alloc(IdExAlloc::Jal_.into()), ExConnect::Jal_.into());
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::BranchEn.into()),
            ExConnect::BranchEn.into(),
        );
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::PcSel.into()),
            ExConnect::PcSel.into(),
        );
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::ImmSel.into()),
            ExConnect::ImmSel.into(),
        );
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::AluCtrl.into()),
            ExConnect::AluCtrl.into(),
        );
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::BranchType.into()),
            ExConnect::BranchType.into(),
        );
        ex_stage.connect(id_ex.alloc(IdExAlloc::Pc.into()), ExConnect::Pc.into());
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::Rs1Data.into()),
            ExConnect::Rs1Data.into(),
        );
        ex_stage.connect(
            id_ex.alloc(IdExAlloc::Rs2Data.into()),
            ExConnect::Rs2Data.into(),
        );
        ex_stage.connect(id_ex.alloc(IdExAlloc::Imm.into()), ExConnect::Imm.into());
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs1.into()), ExConnect::Rs1.into());
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs2.into()), ExConnect::Rs2.into());
        ex_stage.connect(ex_mem.alloc(ExMemAlloc::Rd.into()), ExConnect::RdMem.into());
        ex_stage.connect(
            ex_mem.alloc(ExMemAlloc::RegWrite.into()),
            ExConnect::RdMemWrite.into(),
        );
        ex_stage.connect(
            mem_wb.alloc(MemWbAlloc::AluRes.into()),
            ExConnect::RdMemData.into(),
        );
        ex_stage.connect(mem_wb.alloc(MemWbAlloc::Rd.into()), ExConnect::RdWb.into());
        ex_stage.connect(
            mem_wb.alloc(MemWbAlloc::RegWrite.into()),
            ExConnect::RdWbWrite.into(),
        );
        ex_stage.connect(
            wb_stage.alloc(WbAlloc::Out.into()),
            ExConnect::RdWbData.into(),
        );
        //set up ex-mem register
        ex_mem.connect(
            id_ex.alloc(IdExAlloc::RegWrite.into()),
            ExMemConnect::RegWrite.into(),
        );
        ex_mem.connect(
            id_ex.alloc(IdExAlloc::WbSel.into()),
            ExMemConnect::WbSel.into(),
        );
        ex_mem.connect(
            id_ex.alloc(IdExAlloc::MemWrite.into()),
            ExMemConnect::MemWrite.into(),
        );
        ex_mem.connect(id_ex.alloc(IdExAlloc::Npc.into()), ExMemConnect::Npc.into());
        ex_mem.connect(
            ex_stage.alloc(ExAlloc::AluRes.into()),
            ExMemConnect::AluRes.into(),
        );
        ex_mem.connect(
            ex_stage.alloc(ExAlloc::Rs2Data.into()),
            ExMemConnect::Rs2Data.into(),
        );
        ex_mem.connect(id_ex.alloc(IdExAlloc::Rd.into()), ExMemConnect::Rd.into());
        ex_mem.connect(
            id_ex.alloc(IdExAlloc::LoadSignal.into()),
            ExMemConnect::MemRead.into(),
        );
        ex_mem.asm.inner.borrow_mut().prev = Some(id_ex.alloc_asm());
        //set up mem stage
        mem_stage.connect(
            ex_mem.alloc(ExMemAlloc::MemWrite.into()),
            MemConnect::Write.into(),
        );
        mem_stage.connect(
            ex_mem.alloc(ExMemAlloc::AluRes.into()),
            MemConnect::Address.into(),
        );
        mem_stage.connect(
            ex_mem.alloc(ExMemAlloc::Rs2Data.into()),
            MemConnect::Data.into(),
        );
        mem_stage.connect(
            ex_mem.alloc(ExMemAlloc::MemRead.into()),
            MemConnect::Read.into(),
        );
        //set up mem-wb register
        mem_wb.connect(
            ex_mem.alloc(ExMemAlloc::RegWrite.into()),
            MemWbConnect::RegWrite.into(),
        );
        mem_wb.connect(
            ex_mem.alloc(ExMemAlloc::WbSel.into()),
            MemWbConnect::WbSel.into(),
        );
        mem_wb.connect(
            ex_mem.alloc(ExMemAlloc::Npc.into()),
            MemWbConnect::Npc.into(),
        );
        mem_wb.connect(
            ex_mem.alloc(ExMemAlloc::AluRes.into()),
            MemWbConnect::AluRes.into(),
        );
        mem_wb.connect(
            mem_stage.alloc(MemAlloc::Out.into()),
            MemWbConnect::MemData.into(),
        );
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::Rd.into()), MemWbConnect::Rd.into());
        mem_wb.asm.inner.borrow_mut().prev = Some(ex_mem.alloc_asm());
        //set up wb stage
        wb_stage.connect(
            mem_wb.alloc(MemWbAlloc::WbSel.into()),
            WbConnect::WbSel.into(),
        );
        wb_stage.connect(mem_wb.alloc(MemWbAlloc::Npc.into()), WbConnect::Npc.into());
        wb_stage.connect(
            mem_wb.alloc(MemWbAlloc::AluRes.into()),
            WbConnect::AluRes.into(),
        );
        wb_stage.connect(
            mem_wb.alloc(MemWbAlloc::MemData.into()),
            WbConnect::MemData.into(),
        );
        //set up hazard unit
        hazard.connect(
            id_stage.alloc(IdAlloc::Rs1.into()),
            HazardConnect::IdRs1.into(),
        );
        hazard.connect(
            id_stage.alloc(IdAlloc::Rs2.into()),
            HazardConnect::IdRs2.into(),
        );
        hazard.connect(
            ex_mem.alloc(ExMemAlloc::Rd.into()),
            HazardConnect::ExRd.into(),
        );
        hazard.connect(
            id_ex.alloc(IdExAlloc::LoadSignal.into()),
            HazardConnect::LoadSignal.into(),
        );
        hazard.connect(
            ex_stage.alloc(ExAlloc::BranchSel.into()),
            HazardConnect::NpcSel.into(),
        );
        //set up consts
        let mut consts = ConstsBuilder::default();
        consts.push(1);
        consts.push(1);
        consts.push(0);
        consts.push(1);
        consts.push(0);
        consts.push(0);
        id_ex.connect(consts.alloc(0), IdExConnect::Enable.into());
        ex_mem.connect(consts.alloc(1), ExMemConnect::Ebable.into());
        ex_mem.connect(consts.alloc(2), ExMemConnect::Clear.into());
        mem_wb.connect(consts.alloc(3), MemWbConnect::Enable.into());
        mem_wb.connect(consts.alloc(4), MemWbConnect::Clear.into());
        if_id.connect(consts.alloc(5), IfIdConnect::Clear.into());
        //build
        Self {
            if_stage,
            id_stage,
            ex_stage,
            mem_stage,
            wb_stage,
            hazard,
            if_id,
            id_ex,
            ex_mem,
            mem_wb,
        }
    }
    pub fn new(inst_mem: Vec<u32>, asm_mem: Vec<String>) -> Self {
        Self::connect(inst_mem, asm_mem)
    }
}

impl Builder for Rv32iBuilder {
    fn alloc(&mut self, id: usize) -> crate::component::PortRef {
        unimplemented!()
    }
    fn connect(&mut self, pin: crate::component::PortRef, id: usize) {
        unimplemented!()
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        Some(ControlRef::from(ControlShared::new(Rv32i {
            if_stage: self.if_stage.build().unwrap(),
            id_stage: self.id_stage.build().unwrap(),
            mem_stage: self.mem_stage.build().unwrap(),
            if_id: self.if_id.build().unwrap(),
            id_ex: self.id_ex.build().unwrap(),
            ex_mem: self.ex_mem.build().unwrap(),
            mem_wb: self.mem_wb.build().unwrap(),
            hazard: self.hazard.build().unwrap(),
        })))
    }
}

pub struct Rv32i {
    pub if_stage: ControlRef,
    pub id_stage: ControlRef,
    pub mem_stage: ControlRef,
    pub if_id: ControlRef,
    pub id_ex: ControlRef,
    pub ex_mem: ControlRef,
    pub mem_wb: ControlRef,
    pub hazard: ControlRef,
}

impl Control for Rv32i {
    fn rasing_edge(&mut self) {
        self.if_stage.rasing_edge();
        self.if_id.rasing_edge();
        self.id_stage.rasing_edge();
        self.id_ex.rasing_edge();
        self.ex_mem.rasing_edge();
        self.mem_stage.rasing_edge();
        self.mem_wb.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.if_stage.falling_edge();
        self.if_id.falling_edge();
        self.id_stage.falling_edge();
        self.id_ex.falling_edge();
        self.ex_mem.falling_edge();
        self.mem_stage.falling_edge();
        self.mem_wb.falling_edge();
    }
    fn debug(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            self.if_id.debug(),
            self.id_ex.debug(),
            self.ex_mem.debug(),
            self.mem_wb.debug(),
            self.hazard.debug()
        )
    }
}

pub enum IfIdAlloc {
    Npc,
    Pc,
    Instruction,
}
impl From<IfIdAlloc> for usize {
    fn from(id: IfIdAlloc) -> Self {
        match id {
            IfIdAlloc::Npc => 0,
            IfIdAlloc::Pc => 1,
            IfIdAlloc::Instruction => 2,
        }
    }
}

pub enum IfIdConnect {
    Npc = 0,
    Pc = 1,
    Instruction = 2,
    Enable = 3,
    Clear = 4,
}

impl From<IfIdConnect> for usize {
    fn from(id: IfIdConnect) -> Self {
        match id {
            IfIdConnect::Npc => 0,
            IfIdConnect::Pc => 1,
            IfIdConnect::Instruction => 2,
            IfIdConnect::Enable => 3,
            IfIdConnect::Clear => 4,
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
impl IfIdBuilder {
    pub fn alloc_asm(&self) -> Rc<RefCell<AsmReg>> {
        self.asm.alloc_asm()
    }
}

impl Builder for IfIdBuilder {
    fn alloc(&mut self, id: usize) -> crate::component::PortRef {
        match id {
            0 => self.npc.alloc(RegAlloc::Out.into()),
            1 => self.pc.alloc(RegAlloc::Out.into()),
            2 => self.instruction.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: crate::component::PortRef, id: usize) {
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
    fn build(self) -> Option<crate::component::ControlRef> {
        Some(ControlRef::from(ControlShared::new(IfId {
            npc: self.npc.build().unwrap(),
            pc: self.pc.build().unwrap(),
            instruction: self.instruction.build().unwrap(),
            asm: self.asm.inner,
        })))
    }
}

pub struct IfId {
    pub npc: ControlRef,
    pub pc: ControlRef,
    pub instruction: ControlRef,
    pub asm: Rc<RefCell<AsmReg>>,
}
impl Control for IfId {
    fn rasing_edge(&mut self) {
        self.npc.rasing_edge();
        self.pc.rasing_edge();
        self.instruction.rasing_edge();
        self.asm.borrow_mut().rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.npc.falling_edge();
        self.pc.falling_edge();
        self.instruction.falling_edge();
        self.asm.borrow_mut().falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "IF/ID : {}\nNPC\t\t: {:8} PC\t\t: {:8} INST\t\t: {:8}",
            self.asm.borrow().inst,
            self.npc.debug(),
            self.pc.debug(),
            self.instruction.debug()
        )
    }
}

pub enum IdExAlloc {
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
impl From<IdExAlloc> for usize {
    fn from(id: IdExAlloc) -> Self {
        match id {
            IdExAlloc::RegWrite => 0,
            IdExAlloc::WbSel => 1,
            IdExAlloc::MemWrite => 2,
            IdExAlloc::Jal_ => 4,
            IdExAlloc::BranchEn => 5,
            IdExAlloc::PcSel => 6,
            IdExAlloc::ImmSel => 7,
            IdExAlloc::AluCtrl => 8,
            IdExAlloc::BranchType => 9,
            IdExAlloc::Npc => 10,
            IdExAlloc::Pc => 11,
            IdExAlloc::Rs1Data => 12,
            IdExAlloc::Rs2Data => 13,
            IdExAlloc::Imm => 14,
            IdExAlloc::Rs1 => 15,
            IdExAlloc::Rd => 16,
            IdExAlloc::Rs2 => 17,
            IdExAlloc::Opco => 18,
            IdExAlloc::LoadSignal => 18,
        }
    }
}

pub enum IdExConnect {
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
impl From<IdExConnect> for usize {
    fn from(id: IdExConnect) -> Self {
        match id {
            IdExConnect::RegWrite => 0,
            IdExConnect::WbSel => 1,
            IdExConnect::MemWrite => 2,
            IdExConnect::Jal_ => 4,
            IdExConnect::BranchEn => 5,
            IdExConnect::PcSel => 6,
            IdExConnect::ImmSel => 7,
            IdExConnect::AluCtrl => 8,
            IdExConnect::BranchType => 9,
            IdExConnect::Npc => 10,
            IdExConnect::Pc => 11,
            IdExConnect::Rs1Data => 12,
            IdExConnect::Rs2Data => 13,
            IdExConnect::Imm => 14,
            IdExConnect::Rs1 => 15,
            IdExConnect::Rd => 16,
            IdExConnect::Rs2 => 17,
            IdExConnect::Opcode => 18,
            IdExConnect::Enable => 19,
            IdExConnect::Clear => 20,
            IdExConnect::LoadSignal => 21,
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
impl IdExBuilder {
    pub fn alloc_asm(&self) -> Rc<RefCell<AsmReg>> {
        self.asm.alloc_asm()
    }
}
impl Builder for IdExBuilder {
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
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(ControlShared::new(IdEx {
            reg_write: self.reg_write.build().unwrap(),
            wb_sel: self.wb_sel.build().unwrap(),
            mem_write: self.mem_write.build().unwrap(),
            jal_: self.jal_.build().unwrap(),
            branch_sel: self.branch_sel.build().unwrap(),
            pc_sel: self.pc_sel.build().unwrap(),
            imm_sel: self.imm_sel.build().unwrap(),
            alu_ctrl: self.alu_ctrl.build().unwrap(),
            branch_type: self.branch_type.build().unwrap(),
            npc: self.npc.build().unwrap(),
            pc: self.pc.build().unwrap(),
            rs1_data: self.rs1_data.build().unwrap(),
            rs2_data: self.rs2_data.build().unwrap(),
            imm: self.imm.build().unwrap(),
            rs1: self.rs1.build().unwrap(),
            rd: self.rd.build().unwrap(),
            rs2: self.rs2.build().unwrap(),
            opco: self.opco.build().unwrap(),
            load_signal: self.load_signal.build().unwrap(),
            asm: self.asm.inner,
        })))
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
    pub asm: Rc<RefCell<AsmReg>>,
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
        self.asm.borrow_mut().rasing_edge();
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
        self.asm.borrow_mut().falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "ID/EX : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} MEM_WRITE\t: {:8} JAL_\t\t: {:8} BRANCH_EN\t: {:8}\nPC_SEL\t\t: {:8} IMM_SEL\t: {:8} ALU_CTRL\t: {:8} BRANCH_TYPE\t: {:8} NPC\t\t: {:8}\nPC\t\t: {:8} RS1_DATA\t: {:8} RS2_DATA\t: {:8} IMM\t\t: {:8} RS1\t\t: {:8}\nRD\t\t: {:8} RS2\t\t: {:8} OPCODE\t: {:8} LOAD_SIGNAL\t: {}",
            self.asm.borrow().inst,
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

pub enum ExMemAlloc {
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

impl From<ExMemAlloc> for usize {
    fn from(id: ExMemAlloc) -> Self {
        match id {
            ExMemAlloc::RegWrite => 0,
            ExMemAlloc::WbSel => 1,
            ExMemAlloc::MemWrite => 2,
            ExMemAlloc::Npc => 4,
            ExMemAlloc::AluRes => 5,
            ExMemAlloc::Rs2Data => 6,
            ExMemAlloc::Rd => 7,
            ExMemAlloc::MemRead => 8,
        }
    }
}

pub enum ExMemConnect {
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

impl From<ExMemConnect> for usize {
    fn from(id: ExMemConnect) -> Self {
        match id {
            ExMemConnect::RegWrite => 0,
            ExMemConnect::WbSel => 1,
            ExMemConnect::MemWrite => 2,
            ExMemConnect::Npc => 4,
            ExMemConnect::AluRes => 5,
            ExMemConnect::Rs2Data => 6,
            ExMemConnect::Rd => 7,
            ExMemConnect::Ebable => 8,
            ExMemConnect::Clear => 9,
            ExMemConnect::MemRead => 10,
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
impl ExMemBuilder {
    pub fn alloc_asm(&self) -> Rc<RefCell<AsmReg>> {
        self.asm.alloc_asm()
    }
}
impl Builder for ExMemBuilder {
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
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(ControlShared::new(ExMem {
            reg_write: self.reg_write.build().unwrap(),
            wb_sel: self.wb_sel.build().unwrap(),
            mem_write: self.mem_write.build().unwrap(),
            npc: self.npc.build().unwrap(),
            alu_res: self.alu_res.build().unwrap(),
            rs2_data: self.rs2_data.build().unwrap(),
            rd: self.rd.build().unwrap(),
            mem_read: self.mem_read.build().unwrap(),
            asm: self.asm.inner,
        })))
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
    pub asm: Rc<RefCell<AsmReg>>,
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
        self.asm.borrow_mut().rasing_edge();
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
        self.asm.borrow_mut().falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "EX/MEM : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} MEM_WRITE\t: {:8} NPC\t\t: {:8} ALU_RES\t: {:8}\nRS2_DATA\t: {:8} RD\t\t: {}",
            self.asm.borrow().inst,
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

pub enum MemWbAlloc {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
}

impl From<MemWbAlloc> for usize {
    fn from(id: MemWbAlloc) -> Self {
        match id {
            MemWbAlloc::RegWrite => 0,
            MemWbAlloc::WbSel => 1,
            MemWbAlloc::Npc => 2,
            MemWbAlloc::AluRes => 3,
            MemWbAlloc::MemData => 4,
            MemWbAlloc::Rd => 5,
        }
    }
}

pub enum MemWbConnect {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
    Enable = 6,
    Clear = 7,
}

impl From<MemWbConnect> for usize {
    fn from(id: MemWbConnect) -> Self {
        match id {
            MemWbConnect::RegWrite => 0,
            MemWbConnect::WbSel => 1,
            MemWbConnect::Npc => 2,
            MemWbConnect::AluRes => 3,
            MemWbConnect::MemData => 4,
            MemWbConnect::Rd => 5,
            MemWbConnect::Enable => 6,
            MemWbConnect::Clear => 7,
        }
    }
}

#[derive(Default)]
pub struct MemWbBuilder {
    pub reg_write: RegBuilder,
    pub wb_sel: RegBuilder,
    pub npc: RegBuilder,
    pub alu_res: RegBuilder,
    pub mem_data: RegBuilder,
    pub rd: RegBuilder,
    pub asm: AsmRegBuilder,
}
impl MemWbBuilder {
    pub fn alloc_asm(&self) -> Rc<RefCell<AsmReg>> {
        self.asm.alloc_asm()
    }
}
impl Builder for MemWbBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => self.reg_write.alloc(RegAlloc::Out.into()),
            1 => self.wb_sel.alloc(RegAlloc::Out.into()),
            2 => self.npc.alloc(RegAlloc::Out.into()),
            3 => self.alu_res.alloc(RegAlloc::Out.into()),
            4 => self.mem_data.alloc(RegAlloc::Out.into()),
            5 => self.rd.alloc(RegAlloc::Out.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.reg_write.connect(pin, RegConnect::In.into()),
            1 => self.wb_sel.connect(pin, RegConnect::In.into()),
            2 => self.npc.connect(pin, RegConnect::In.into()),
            3 => self.alu_res.connect(pin, RegConnect::In.into()),
            4 => self.mem_data.connect(pin, RegConnect::In.into()),
            5 => self.rd.connect(pin, RegConnect::In.into()),
            6 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Enable.into());
                self.npc.connect(pin.clone(), RegConnect::Enable.into());
                self.alu_res.connect(pin.clone(), RegConnect::Enable.into());
                self.mem_data
                    .connect(pin.clone(), RegConnect::Enable.into());
                self.rd.connect(pin, RegConnect::Enable.into());
            }
            7 => {
                self.reg_write
                    .connect(pin.clone(), RegConnect::Clear.into());
                self.wb_sel.connect(pin.clone(), RegConnect::Clear.into());
                self.npc.connect(pin.clone(), RegConnect::Clear.into());
                self.alu_res.connect(pin.clone(), RegConnect::Clear.into());
                self.mem_data.connect(pin.clone(), RegConnect::Clear.into());
                self.rd.connect(pin, RegConnect::Clear.into());
            }
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(ControlShared::new(MemWb {
            reg_write: self.reg_write.build().unwrap(),
            wb_sel: self.wb_sel.build().unwrap(),
            npc: self.npc.build().unwrap(),
            alu_res: self.alu_res.build().unwrap(),
            mem_data: self.mem_data.build().unwrap(),
            rd: self.rd.build().unwrap(),
            asm: self.asm.inner,
        })))
    }
}

pub struct MemWb {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub npc: ControlRef,
    pub alu_res: ControlRef,
    pub mem_data: ControlRef,
    pub rd: ControlRef,
    pub asm: Rc<RefCell<AsmReg>>,
}

impl Control for MemWb {
    fn rasing_edge(&mut self) {
        self.reg_write.rasing_edge();
        self.wb_sel.rasing_edge();
        self.npc.rasing_edge();
        self.alu_res.rasing_edge();
        self.mem_data.rasing_edge();
        self.rd.rasing_edge();
        self.asm.borrow_mut().rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.reg_write.falling_edge();
        self.wb_sel.falling_edge();
        self.npc.falling_edge();
        self.alu_res.falling_edge();
        self.mem_data.falling_edge();
        self.rd.falling_edge();
        self.asm.borrow_mut().falling_edge();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!(
            "MEM/WB : {}\nREG_WRITE\t: {:8} WB_SEL\t: {:8} NPC\t\t: {:8} ALU_RES\t: {:8} MEM_DATA\t: {:8}\nRD\t\t: {}",
            self.asm.borrow().inst,
            self.reg_write.debug(),
            self.wb_sel.debug(),
            self.npc.debug(),
            self.alu_res.debug(),
            self.mem_data.debug(),
            self.rd.debug()
        )
    }
}
