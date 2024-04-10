use super::{BitBuilder, Builder, Component, ComponentRef, ComponentShared};
pub enum Alloc {
    BranchType = 0,
    AluOp = 1,
    ImmSel = 2,
    PcSel = 3,
    BranchSel = 4,
    MemWrite = 5,
    //
    Jal_ = 7,
    WbSel = 8,
    RegWrite = 9,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::BranchType => 0,
            Alloc::AluOp => 1,
            Alloc::ImmSel => 2,
            Alloc::PcSel => 3,
            Alloc::BranchSel => 4,
            Alloc::MemWrite => 5,
            Alloc::Jal_ => 7,
            Alloc::WbSel => 8,
            Alloc::RegWrite => 9,
        }
    }
}
pub struct ControlBuilder {
    branch_type: BitBuilder,
    alu_ctl: ComponentShared<AluCtl>,
    imm_sel: ComponentShared<ImmSel>,
    pc_sel: ComponentShared<PcSel>,
    branch_sel: ComponentShared<BranchSel>,
    mem_write: ComponentShared<MemWrite>,
    jal_: ComponentShared<Jal_>,
    wb_sel: ComponentShared<WbSel>,
    reg_write: ComponentShared<RegWrite>,
}

impl ControlBuilder {
    pub fn new() -> Self {
        Self {
            branch_type: BitBuilder::new((12, 14)),
            alu_ctl: ComponentShared::new(AluCtl::default()),
            imm_sel: ComponentShared::new(ImmSel::default()),
            pc_sel: ComponentShared::new(PcSel::default()),
            branch_sel: ComponentShared::new(BranchSel::default()),
            mem_write: ComponentShared::new(MemWrite::default()),
            jal_: ComponentShared::new(Jal_::default()),
            wb_sel: ComponentShared::new(WbSel::default()),
            reg_write: ComponentShared::new(RegWrite::default()),
        }
    }
}

impl Builder for ControlBuilder {
    fn alloc(&mut self, id: usize) -> ComponentRef {
        match id {
            0 => ComponentRef::from(self.branch_type.inner.clone()),
            1 => ComponentRef::from(self.alu_ctl.clone()),
            2 => ComponentRef::from(self.imm_sel.clone()),
            3 => ComponentRef::from(self.pc_sel.clone()),
            4 => ComponentRef::from(self.branch_sel.clone()),
            5 => ComponentRef::from(self.mem_write.clone()),
            7 => ComponentRef::from(self.jal_.clone()),
            8 => ComponentRef::from(self.wb_sel.clone()),
            9 => ComponentRef::from(self.reg_write.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        None
    }
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        assert!(id == 0);
        self.alu_ctl.borrow_mut().input = Some(pin.clone());
        self.imm_sel.borrow_mut().input = Some(pin.clone());
        self.pc_sel.borrow_mut().input = Some(pin.clone());
        self.branch_sel.borrow_mut().input = Some(pin.clone());
        self.mem_write.borrow_mut().input = Some(pin.clone());
        self.jal_.borrow_mut().input = Some(pin.clone());
        self.wb_sel.borrow_mut().input = Some(pin.clone());
        self.reg_write.borrow_mut().input = Some(pin.clone());
    }
}
pub struct Control {}
impl Component for Control {
    fn read(&self) -> u32 {
        unimplemented!()
    }
}
#[derive(Default)]
pub struct Jal_ {
    pub input: Option<ComponentRef>,
}
impl Component for Jal_ {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b0000_0000_0000_0000_0000_0000_0111_1111 & input;
        if opcode == 0b110_1111 || opcode == 0b110_0111 {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct BranchSel {
    pub input: Option<ComponentRef>,
}

impl Component for BranchSel {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b0000_0000_0000_0000_0000_0000_0111_1111 & input;
        if opcode == 0b110_0011 {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct AluCtl {
    pub input: Option<ComponentRef>,
}

impl Component for AluCtl {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b0000_0000_0000_0000_0000_0000_0111_1111 & input;
        match opcode {
            0b011_0011 => ((input << 1) >> 31) | ((input << 17) >> 28), //alu
            0b001_0011 => ((input << 1) >> 31) | ((input << 17) >> 28) | 1, //imm
            0b000_0011 | 0b010_0011 | 0b110_0011 | 0b110_1111 | 0b110_0111 => 0, //load, store, branch, jal, jalr
            _ => {
                unimplemented!();
            }
        }
    }
}

#[derive(Default)]
pub struct ImmSel {
    pub input: Option<ComponentRef>,
}

impl Component for ImmSel {
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
    pub input: Option<ComponentRef>,
}

impl Component for PcSel {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b0000_0000_0000_0000_0000_0000_0111_1111 & input;
        match opcode {
            0b110_0011 | 0b110_1111 | 0b110_0111 => 1, //branch, jal, jalr
            _ => 0,
        }
    }
}

#[derive(Default)]
pub struct MemWrite {
    pub input: Option<ComponentRef>,
}

impl Component for MemWrite {
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
    pub input: Option<ComponentRef>,
}

impl Component for WbSel {
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
    pub input: Option<ComponentRef>,
}

impl Component for RegWrite {
    fn read(&self) -> u32 {
        let input = match self.input {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let opcode = 0b0000_0000_0000_0000_0000_0000_0111_1111 & input;
        match opcode {
            0b011_0011 | 0b000_0011 | 0b001_0011 | 0b110_0011 | 0b110_1111 | 0b110_0111 => 1, //alu, load, imm, branch, jal, jalr
            _ => 0, //lui, auipc todo: add more
        }
    }
}
