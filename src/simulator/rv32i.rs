use crate::common::abi::*;
use crate::common::build::*;
use crate::config::Program;
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
use mem_stage::build::*;
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
    pub asm: AsmMemBuilder,
    pub pgbak: Program,
}

impl Rv32iBuilder {
    fn connect(pg: crate::config::Program) -> Self {
        // let inst_mem = inst_mem.into_iter().flat_map(|x| x.to_ne_bytes()).collect();
        let mut consts = ConstsBuilder::default();
        let mut if_stage = IfStageBuilder::new(pg.entry as u32, pg.start as u32, pg.insts.clone());
        let mut if_id = IfIdBuilder::default();
        let mut id_stage = IdStageBuilder::new(0x7FFFFFF0);
        let mut id_ex = IdExBuilder::default();
        let mut ex_stage = ExStageBuilder::new();
        let mut mem_stage = MemStageBuilder::default();
        let mut wb_stage = WbStageBuilder::default();
        let mut hazard = HazardBuilder::new();
        let mut ex_mem = ExMemBuilder::default();
        let mut mem_wb = MemWbBuilder::default();
        //first try connect
        //set up id stage
        id_stage.connect(if_id.alloc(IfIdAlloc::Instruction), IdConnect::Inst);
        //set up id-ex register
        id_ex.connect(if_id.alloc(IfIdAlloc::Npc), IdExConnect::Npc);
        id_ex.connect(if_id.alloc(IfIdAlloc::Pc), IdExConnect::Pc);
        //set up ex stage
        ex_stage.connect(id_ex.alloc(IdExAlloc::Jal_), ExConnect::Jal_);
        ex_stage.connect(id_ex.alloc(IdExAlloc::BranchEn), ExConnect::BranchEn);
        ex_stage.connect(id_ex.alloc(IdExAlloc::PcSel), ExConnect::PcSel);
        ex_stage.connect(id_ex.alloc(IdExAlloc::ImmSel), ExConnect::ImmSel);
        ex_stage.connect(id_ex.alloc(IdExAlloc::AluCtrl), ExConnect::AluCtrl);
        ex_stage.connect(id_ex.alloc(IdExAlloc::BranchType), ExConnect::BranchType);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Pc), ExConnect::Pc);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs1Data), ExConnect::Rs1Data);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs2Data), ExConnect::Rs2Data);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Imm), ExConnect::Imm);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs1), ExConnect::Rs1);
        ex_stage.connect(id_ex.alloc(IdExAlloc::Rs2), ExConnect::Rs2);
        //set up ex-mem register
        ex_mem.connect(id_ex.alloc(IdExAlloc::RegWrite), ExMemConnect::RegWrite);
        ex_mem.connect(id_ex.alloc(IdExAlloc::WbSel), ExMemConnect::WbSel);
        ex_mem.connect(id_ex.alloc(IdExAlloc::MemWrite), ExMemConnect::MemWrite);
        ex_mem.connect(id_ex.alloc(IdExAlloc::Npc), ExMemConnect::Npc);
        ex_mem.connect(id_ex.alloc(IdExAlloc::Rd), ExMemConnect::Rd);
        ex_mem.connect(id_ex.alloc(IdExAlloc::LoadSignal), ExMemConnect::MemRead);
        //set up mem stage
        mem_stage.connect(ex_mem.alloc(ExMemAlloc::MemWrite), MemStageConnect::WriteEn);
        mem_stage.connect(ex_mem.alloc(ExMemAlloc::AluRes), MemStageConnect::Addr);
        mem_stage.connect(ex_mem.alloc(ExMemAlloc::Rs2Data), MemStageConnect::Data);
        mem_stage.connect(ex_mem.alloc(ExMemAlloc::MemRead), MemStageConnect::ReadEn);
        //set up mem-wb register
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::RegWrite), MemWbConnect::RegWrite);
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::WbSel), MemWbConnect::WbSel);
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::Npc), MemWbConnect::Npc);
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::AluRes), MemWbConnect::AluRes);
        mem_wb.connect(ex_mem.alloc(ExMemAlloc::Rd), MemWbConnect::Rd);
        //set up wb stage
        wb_stage.connect(mem_wb.alloc(MemWbAlloc::WbSel), WbConnect::WbSel);
        wb_stage.connect(mem_wb.alloc(MemWbAlloc::Npc), WbConnect::Npc);
        wb_stage.connect(mem_wb.alloc(MemWbAlloc::AluRes), WbConnect::AluRes);
        wb_stage.connect(mem_wb.alloc(MemWbAlloc::MemData), WbConnect::MemData);
        //set up hazard unit
        hazard.connect(id_stage.alloc(IdAlloc::Rs1), HazardConnect::IdRs1);
        hazard.connect(id_stage.alloc(IdAlloc::Rs2), HazardConnect::IdRs2);
        hazard.connect(id_ex.alloc(IdExAlloc::Rd), HazardConnect::ExRd);
        hazard.connect(
            id_ex.alloc(IdExAlloc::LoadSignal),
            HazardConnect::LoadSignal,
        );
        //second try connect
        //set up if stage
        if_stage.connect(hazard.alloc(HazardAlloc::PcEnable), IfConnect::PcEnable);
        //set up if-id register
        if_id.connect(hazard.alloc(HazardAlloc::IfIdEnable), IfIdConnect::Enable);
        //set up id stage
        id_stage.connect(mem_wb.alloc(MemWbAlloc::RegWrite), IdConnect::RegWrite);
        id_stage.connect(mem_wb.alloc(MemWbAlloc::Rd), IdConnect::Rd);
        id_stage.connect(wb_stage.alloc(WbAlloc::Out), IdConnect::RdData);
        //set up id-ex register
        id_ex.connect(id_stage.alloc(IdAlloc::RegWrite), IdExConnect::RegWrite);
        id_ex.connect(id_stage.alloc(IdAlloc::WbSel), IdExConnect::WbSel);
        id_ex.connect(id_stage.alloc(IdAlloc::MemWrite), IdExConnect::MemWrite);
        id_ex.connect(id_stage.alloc(IdAlloc::Load), IdExConnect::LoadSignal);
        id_ex.connect(id_stage.alloc(IdAlloc::Jal_), IdExConnect::Jal_);
        id_ex.connect(id_stage.alloc(IdAlloc::BranchEn), IdExConnect::BranchEn);
        id_ex.connect(id_stage.alloc(IdAlloc::PcSel), IdExConnect::PcSel);
        id_ex.connect(id_stage.alloc(IdAlloc::ImmSel), IdExConnect::ImmSel);
        id_ex.connect(id_stage.alloc(IdAlloc::AluCtrl), IdExConnect::AluCtrl);
        id_ex.connect(id_stage.alloc(IdAlloc::BranchType), IdExConnect::BranchType);
        id_ex.connect(id_stage.alloc(IdAlloc::Rs1Data), IdExConnect::Rs1Data);
        id_ex.connect(id_stage.alloc(IdAlloc::Rs2Data), IdExConnect::Rs2Data);
        id_ex.connect(id_stage.alloc(IdAlloc::Imm), IdExConnect::Imm);
        id_ex.connect(id_stage.alloc(IdAlloc::Rs1), IdExConnect::Rs1);
        id_ex.connect(id_stage.alloc(IdAlloc::Rd), IdExConnect::Rd);
        id_ex.connect(id_stage.alloc(IdAlloc::Rs2), IdExConnect::Rs2);
        id_ex.connect(id_stage.alloc(IdAlloc::Opcode), IdExConnect::Opcode);
        //set up ex stage
        ex_stage.connect(ex_mem.alloc(ExMemAlloc::Rd), ExConnect::RdMem);
        ex_stage.connect(ex_mem.alloc(ExMemAlloc::RegWrite), ExConnect::RdMemWrite);
        ex_stage.connect(ex_mem.alloc(ExMemAlloc::AluRes), ExConnect::RdMemData);
        ex_stage.connect(mem_wb.alloc(MemWbAlloc::Rd), ExConnect::RdWb);
        ex_stage.connect(mem_wb.alloc(MemWbAlloc::RegWrite), ExConnect::RdWbWrite);
        ex_stage.connect(wb_stage.alloc(WbAlloc::Out), ExConnect::RdWbData);
        //set up ex-mem register
        ex_mem.connect(ex_stage.alloc(ExAlloc::AluRes), ExMemConnect::AluRes);
        ex_mem.connect(ex_stage.alloc(ExAlloc::Rs2Data), ExMemConnect::Rs2Data);
        //set up mem-wb register
        mem_wb.connect(mem_stage.alloc(MemStageAlloc::Out), MemWbConnect::MemData);
        //set up hazard unit
        hazard.connect(ex_stage.alloc(ExAlloc::BranchSel), HazardConnect::NpcSel);
        //third try connect
        //set up if stage
        if_stage.connect(ex_stage.alloc(ExAlloc::BranchSel), IfConnect::NpcSel);
        if_stage.connect(ex_stage.alloc(ExAlloc::AluRes), IfConnect::Npc);
        //set up if-id register
        if_id.connect(if_stage.alloc(IfAlloc::Npc), IfIdConnect::Npc);
        if_id.connect(if_stage.alloc(IfAlloc::Pc), IfIdConnect::Pc);
        if_id.connect(if_stage.alloc(IfAlloc::Imem), IfIdConnect::Instruction);
        //set up consts
        if_id.connect(ex_stage.alloc(ExAlloc::BranchSel), IfIdConnect::Clear);
        id_ex.connect(consts.alloc(ConstsAlloc::Out(1)), IdExConnect::Enable);
        id_ex.connect(hazard.alloc(HazardAlloc::IdExClear), IdExConnect::Clear);
        ex_mem.connect(consts.alloc(ConstsAlloc::Out(1)), ExMemConnect::Ebable);
        ex_mem.connect(consts.alloc(ConstsAlloc::Out(0)), ExMemConnect::Clear);
        mem_wb.connect(consts.alloc(ConstsAlloc::Out(1)), MemWbConnect::Enable);
        mem_wb.connect(consts.alloc(ConstsAlloc::Out(0)), MemWbConnect::Clear);
        //asm
        let mut asm = AsmMemBuilder::new(pg.entry, pg.asm.clone());
        asm.connect(if_stage.npc_mux.alloc(MuxAlloc::Out), AsmConnect::Address);
        asm.connect(hazard.alloc(HazardAlloc::PcEnable), AsmConnect::IfEn);
        asm.connect(hazard.alloc(HazardAlloc::IfIdEnable), AsmConnect::IdEn);
        asm.connect(hazard.alloc(HazardAlloc::IdExClear), AsmConnect::ExClr);
        asm.connect(ex_stage.alloc(ExAlloc::BranchSel), AsmConnect::IdClr);
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
            asm,
            pgbak: pg,
        }
    }
    pub fn new(pg: Program) -> Self {
        Self::connect(pg)
    }
    pub fn slf_build(self) -> Rv32i {
        Rv32i {
            if_stage: self.if_stage.build(),
            id_stage: self.id_stage.build(),
            mem_stage: self.mem_stage.build(),
            if_id: self.if_id.build(),
            id_ex: self.id_ex.build(),
            ex: self.ex_stage.build(),
            ex_mem: self.ex_mem.build(),
            mem_wb: self.mem_wb.build(),
            hazard: self.hazard.build(),
            asm: self.asm.build(),
            pgbak: self.pgbak,
        }
    }
}

#[derive(Debug)]
pub struct Rv32i {
    pub pgbak: Program,
    pub if_stage: ControlRef,
    pub id_stage: ControlRef,
    pub mem_stage: ControlRef,
    pub if_id: ControlRef,
    pub id_ex: ControlRef,
    pub ex: ControlRef,
    pub ex_mem: ControlRef,
    pub mem_wb: ControlRef,
    pub hazard: ControlRef,
    pub asm: AsmPortRef,
}
impl Rv32i {
    pub fn reset(&self) -> Rv32i {
        Rv32iBuilder::connect(self.pgbak.clone()).slf_build()
    }
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
        self.asm.rasing_edge();
        self.hazard.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.if_stage.falling_edge();
        self.if_id.falling_edge();
        self.id_stage.falling_edge();
        self.id_ex.falling_edge();
        self.ex_mem.falling_edge();
        self.mem_stage.falling_edge();
        self.mem_wb.falling_edge();
        self.asm.falling_edge();
        self.hazard.falling_edge();
    }
}
