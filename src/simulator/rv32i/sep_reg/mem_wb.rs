use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
}

pub enum Connect {
    RegWrite = 0,
    WbSel = 1,
    Npc = 2,
    AluRes = 3,
    MemData = 4,
    Rd = 5,
    Enable = 6,
    Clear = 7,
}

#[derive(Default)]
pub struct MemWbBuilder {
    pub reg_write: RegBuilder,
    pub wb_sel: RegBuilder,
    pub npc: RegBuilder,
    pub alu_res: RegBuilder,
    pub mem_data: RegBuilder,
    pub rd: RegBuilder,
}
impl ControlBuilder for MemWbBuilder {
    fn build(self) -> ControlRef {
        MemWb {
            reg_write: self.reg_write.build(),
            wb_sel: self.wb_sel.build(),
            npc: self.npc.build(),
            alu_res: self.alu_res.build(),
            mem_data: self.mem_data.build(),
            rd: self.rd.build(),
        }
        .into()
    }
}
impl PortBuilder for MemWbBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::RegWrite => self.reg_write.alloc(RegAlloc::Out),
            Alloc::WbSel => self.wb_sel.alloc(RegAlloc::Out),
            Alloc::Npc => self.npc.alloc(RegAlloc::Out),
            Alloc::AluRes => self.alu_res.alloc(RegAlloc::Out),
            Alloc::MemData => self.mem_data.alloc(RegAlloc::Out),
            Alloc::Rd => self.rd.alloc(RegAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::RegWrite => self.reg_write.connect(pin, RegConnect::In),
            Connect::WbSel => self.wb_sel.connect(pin, RegConnect::In),
            Connect::Npc => self.npc.connect(pin, RegConnect::In),
            Connect::AluRes => self.alu_res.connect(pin, RegConnect::In),
            Connect::MemData => self.mem_data.connect(pin, RegConnect::In),
            Connect::Rd => self.rd.connect(pin, RegConnect::In),
            Connect::Enable => {
                self.reg_write.connect(pin.clone(), RegConnect::Enable);
                self.wb_sel.connect(pin.clone(), RegConnect::Enable);
                self.npc.connect(pin.clone(), RegConnect::Enable);
                self.alu_res.connect(pin.clone(), RegConnect::Enable);
                self.mem_data.connect(pin.clone(), RegConnect::Enable);
                self.rd.connect(pin, RegConnect::Enable);
            }
            Connect::Clear => {
                self.reg_write.connect(pin.clone(), RegConnect::Clear);
                self.wb_sel.connect(pin.clone(), RegConnect::Clear);
                self.npc.connect(pin.clone(), RegConnect::Clear);
                self.alu_res.connect(pin.clone(), RegConnect::Clear);
                self.mem_data.connect(pin.clone(), RegConnect::Clear);
                self.rd.connect(pin, RegConnect::Clear);
            }
        }
    }
}

#[derive(Debug)]
pub struct MemWb {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub npc: ControlRef,
    pub alu_res: ControlRef,
    pub mem_data: ControlRef,
    pub rd: ControlRef,
}

impl Control for MemWb {
    fn rasing_edge(&mut self) {
        self.reg_write.rasing_edge();
        self.wb_sel.rasing_edge();
        self.npc.rasing_edge();
        self.alu_res.rasing_edge();
        self.mem_data.rasing_edge();
        self.rd.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.reg_write.falling_edge();
        self.wb_sel.falling_edge();
        self.npc.falling_edge();
        self.alu_res.falling_edge();
        self.mem_data.falling_edge();
        self.rd.falling_edge();
    }
    fn inout(&self) -> Vec<(String, u32, u32)> {
        let mut res = Vec::new();
        res.push((
            "reg_write".to_string(),
            self.reg_write.output()[0].1,
            self.reg_write.output()[0].1,
        ));
        res.push((
            "wb_sel".to_string(),
            self.wb_sel.output()[0].1,
            self.wb_sel.output()[0].1,
        ));
        res.push((
            "npc".to_string(),
            self.npc.output()[0].1,
            self.npc.output()[0].1,
        ));
        res.push((
            "alu_res".to_string(),
            self.alu_res.output()[0].1,
            self.alu_res.output()[0].1,
        ));
        res.push((
            "mem_data".to_string(),
            self.mem_data.output()[0].1,
            self.mem_data.output()[0].1,
        ));
        res.push((
            "rd".to_string(),
            self.rd.output()[0].1,
            self.rd.output()[0].1,
        ));
        res
    }
}

pub mod build {
    pub use super::Alloc as MemWbAlloc;
    pub use super::Connect as MemWbConnect;
    pub use super::MemWbBuilder;
}
