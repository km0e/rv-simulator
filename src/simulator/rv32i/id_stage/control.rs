use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    BranchType = 0,
    AluCtrl = 1,
    ImmSel = 2,
    PcSel = 3,
    BranchEn = 4,
    MemWrite = 5,
    //
    Jal_ = 7,
    WbSel = 8,
    RegWrite = 9,
    Load = 10,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::BranchType => 0,
            Alloc::AluCtrl => 1,
            Alloc::ImmSel => 2,
            Alloc::PcSel => 3,
            Alloc::BranchEn => 4,
            Alloc::MemWrite => 5,
            Alloc::Jal_ => 7,
            Alloc::WbSel => 8,
            Alloc::RegWrite => 9,
            Alloc::Load => 10,
        }
    }
}
pub struct CtrlSigBuilder {
    branch_type: BitBuilder,
    alu_ctl: PortShared<AluCtl>,
    imm_sel: PortShared<ImmSel>,
    pc_sel: PortShared<PcSel>,
    branch_sel: PortShared<BranchEn>,
    mem_write: PortShared<MemWrite>,
    jal_: PortShared<Jal_>,
    wb_sel: PortShared<WbSel>,
    reg_write: PortShared<RegWrite>,
    load: PortShared<LoadSiganl>,
}

impl CtrlSigBuilder {
    pub fn new() -> Self {
        Self {
            branch_type: BitBuilder::new((12, 14)),
            alu_ctl: PortShared::new(AluCtl::default()),
            imm_sel: PortShared::new(ImmSel::default()),
            pc_sel: PortShared::new(PcSel::default()),
            branch_sel: PortShared::new(BranchEn::default()),
            mem_write: PortShared::new(MemWrite::default()),
            jal_: PortShared::new(Jal_::default()),
            wb_sel: PortShared::new(WbSel::default()),
            reg_write: PortShared::new(RegWrite::default()),
            load: PortShared::new(LoadSiganl::default()),
        }
    }
}

impl PortBuilder for CtrlSigBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => PortRef::from(self.branch_type.inner.clone()),
            1 => PortRef::from(self.alu_ctl.clone()),
            2 => PortRef::from(self.imm_sel.clone()),
            3 => PortRef::from(self.pc_sel.clone()),
            4 => PortRef::from(self.branch_sel.clone()),
            5 => PortRef::from(self.mem_write.clone()),
            7 => PortRef::from(self.jal_.clone()),
            8 => PortRef::from(self.wb_sel.clone()),
            9 => PortRef::from(self.reg_write.clone()),
            10 => PortRef::from(self.load.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        assert!(id == 0);
        self.alu_ctl.borrow_mut().input = Some(pin.clone());
        self.imm_sel.borrow_mut().input = Some(pin.clone());
        self.pc_sel.borrow_mut().input = Some(pin.clone());
        self.branch_sel.borrow_mut().input = Some(pin.clone());
        self.mem_write.borrow_mut().input = Some(pin.clone());
        self.jal_.borrow_mut().input = Some(pin.clone());
        self.wb_sel.borrow_mut().input = Some(pin.clone());
        self.reg_write.borrow_mut().input = Some(pin.clone());
        self.load.borrow_mut().input = Some(pin.clone());
        self.branch_type.connect(pin, 0);
    }
}
pub struct Control {}
impl Port for Control {
    fn read(&self) -> u32 {
        unimplemented!()
    }
}
#[derive(Default)]
pub struct Jal_ {
    pub input: Option<PortRef>,
}
impl Port for Jal_ {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        if opcode == 0b110_1111 || opcode == 0b110_0111 {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct BranchEn {
    pub input: Option<PortRef>,
}

impl Port for BranchEn {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        if opcode == 0b110_0011 {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct AluCtl {
    pub input: Option<PortRef>,
}

impl Port for AluCtl {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b011_0011 | 0b001_0011 => ((input << 1) >> 31) | ((input << 17) >> 28) | 1, //alu
            0b000_0011 | 0b010_0011 | 0b110_0011 | 0b110_1111 | 0b110_0111 => 0b00001,
            _ => {
                0
                // unimplemented!();
            }
        }
    }
}

#[derive(Default)]
pub struct ImmSel {
    pub input: Option<PortRef>,
}

impl Port for ImmSel {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b001_0011 | 0b000_0011 | 0b010_0011 | 0b110_0011 | 0b110_1111 | 0b110_0111 => 1, //alu, load, store, branch, jal, jalr
            _ => 0,
        }
    }
}

#[derive(Default)]
pub struct PcSel {
    pub input: Option<PortRef>,
}

impl Port for PcSel {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b110_0011 | 0b110_1111 | 0b110_0111 => 1, //branch, jal, jalr
            _ => 0,
        }
    }
}

#[derive(Default)]
pub struct MemWrite {
    pub input: Option<PortRef>,
}

impl Port for MemWrite {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b010_0011 => 1, //store
            _ => 0,
        }
    }
}

#[derive(Default)]
pub struct WbSel {
    pub input: Option<PortRef>,
}

impl Port for WbSel {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b110_1111 | 0b110_0111 => 0, //jal, jalr
            0b000_0011 => 2,              //load
            _ => 1,                       //todo: add more
        }
    }
}

#[derive(Default)]
pub struct RegWrite {
    pub input: Option<PortRef>,
}

impl Port for RegWrite {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b011_0011 | 0b000_0011 | 0b001_0011 | 0b110_0011 | 0b110_1111 | 0b110_0111 => 1, //alu, load, imm, branch, jal, jalr
            _ => 0, //lui, auipc todo: add more
        }
    }
}

#[derive(Default)]
pub struct LoadSiganl {
    pub input: Option<PortRef>,
}

impl Port for LoadSiganl {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b111_1111 & input;
        match opcode {
            0b000_0011 => 1, //load
            _ => 0,
        }
    }
}
