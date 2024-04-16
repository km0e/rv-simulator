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
}
impl ControlBuilder for ExMemBuilder {
    fn build(self) -> ControlRef {
        ExMem {
            reg_write: self.reg_write.build(),
            wb_sel: self.wb_sel.build(),
            mem_write: self.mem_write.build(),
            npc: self.npc.build(),
            alu_res: self.alu_res.build(),
            rs2_data: self.rs2_data.build(),
            rd: self.rd.build(),
            mem_read: self.mem_read.build(),
        }
        .into()
    }
}
impl PortBuilder for ExMemBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::RegWrite => self.reg_write.alloc(RegAlloc::Out),
            Alloc::WbSel => self.wb_sel.alloc(RegAlloc::Out),
            Alloc::MemWrite => self.mem_write.alloc(RegAlloc::Out),
            Alloc::Npc => self.npc.alloc(RegAlloc::Out),
            Alloc::AluRes => self.alu_res.alloc(RegAlloc::Out),
            Alloc::Rs2Data => self.rs2_data.alloc(RegAlloc::Out),
            Alloc::Rd => self.rd.alloc(RegAlloc::Out),
            Alloc::MemRead => self.mem_read.alloc(RegAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::RegWrite => self.reg_write.connect(pin, RegConnect::In),
            Connect::WbSel => self.wb_sel.connect(pin, RegConnect::In),
            Connect::MemWrite => self.mem_write.connect(pin, RegConnect::In),
            Connect::Npc => self.npc.connect(pin, RegConnect::In),
            Connect::AluRes => self.alu_res.connect(pin, RegConnect::In),
            Connect::Rs2Data => self.rs2_data.connect(pin, RegConnect::In),
            Connect::Rd => self.rd.connect(pin, RegConnect::In),
            Connect::Ebable => {
                self.reg_write.connect(pin.clone(), RegConnect::Enable);
                self.wb_sel.connect(pin.clone(), RegConnect::Enable);
                self.mem_write.connect(pin.clone(), RegConnect::Enable);
                self.npc.connect(pin.clone(), RegConnect::Enable);
                self.alu_res.connect(pin.clone(), RegConnect::Enable);
                self.rs2_data.connect(pin.clone(), RegConnect::Enable);
                self.rd.connect(pin.clone(), RegConnect::Enable);
                self.mem_read.connect(pin, RegConnect::Enable);
            }
            Connect::Clear => {
                self.reg_write.connect(pin.clone(), RegConnect::Clear);
                self.wb_sel.connect(pin.clone(), RegConnect::Clear);
                self.mem_write.connect(pin.clone(), RegConnect::Clear);
                self.npc.connect(pin.clone(), RegConnect::Clear);
                self.alu_res.connect(pin.clone(), RegConnect::Clear);
                self.rs2_data.connect(pin.clone(), RegConnect::Clear);
                self.rd.connect(pin.clone(), RegConnect::Clear);
                self.mem_read.connect(pin, RegConnect::Clear);
            }
            Connect::MemRead => self.mem_read.connect(pin, RegConnect::In),
        }
    }
}

#[derive(Debug)]
pub struct ExMem {
    pub reg_write: ControlRef,
    pub wb_sel: ControlRef,
    pub mem_write: ControlRef,
    pub mem_read: ControlRef,
    pub npc: ControlRef,
    pub alu_res: ControlRef,
    pub rs2_data: ControlRef,
    pub rd: ControlRef,
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
    }
    fn input(&self) -> Vec<(String, u32)> {
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
            "mem_read".to_string(),
            self.mem_read.input()[0].1,
            self.mem_read.output()[0].1,
        ));
        res.push((
            "npc".to_string(),
            self.npc.input()[0].1,
            self.npc.output()[0].1,
        ));
        res.push((
            "alu_res".to_string(),
            self.alu_res.input()[0].1,
            self.alu_res.output()[0].1,
        ));
        res.push((
            "rs2_data".to_string(),
            self.rs2_data.input()[0].1,
            self.rs2_data.output()[0].1,
        ));
        res.push((
            "rd".to_string(),
            self.rd.input()[0].1,
            self.rd.output()[0].1,
        ));
        res
    }
    fn output(&self) -> Vec<(String, u32)> {
        unimplemented!()
    }
}

pub mod build {
    pub use super::Alloc as ExMemAlloc;
    pub use super::Connect as ExMemConnect;
    pub use super::ExMemBuilder;
}
