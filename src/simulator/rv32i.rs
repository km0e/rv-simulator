use crate::common::abi::*;
use crate::common::build::*;
mod ex_stage;
mod hazard;
mod id_stage;
mod if_stage;
mod mem_stage;
mod sep_reg;
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
use sep_reg::build::*;
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
        if_id.asm_connect(
            if_stage.asm_alloc(AsmAlloc::Out.into()),
            AsmRegConnect::In.into(),
        );
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
        id_ex.asm_connect(
            if_id.asm_alloc(AsmRegAlloc::Out.into()),
            AsmRegConnect::In.into(),
        );
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
        ex_mem.asm_connect(
            id_ex.asm_alloc(AsmRegAlloc::Out.into()),
            AsmRegConnect::In.into(),
        );
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
        mem_wb.asm_connect(
            ex_mem.asm_alloc(AsmRegAlloc::Out.into()),
            AsmRegConnect::In.into(),
        );
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

impl ControlBuilder for Rv32iBuilder {
    fn build(self) -> ControlRef {
        Rv32i {
            if_stage: self.if_stage.build(),
            id_stage: self.id_stage.build(),
            mem_stage: self.mem_stage.build(),
            if_id: self.if_id.build(),
            id_ex: self.id_ex.build(),
            ex_mem: self.ex_mem.build(),
            mem_wb: self.mem_wb.build(),
            hazard: self.hazard.build(),
        }
        .into()
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
    #[cfg(debug_assertions)]
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
