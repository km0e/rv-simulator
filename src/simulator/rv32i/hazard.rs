use std::env::consts;

use crate::common::abi::*;
use crate::common::build::*;

use super::RegBuilder;

pub enum Alloc {
    PcEnable = 0,
    IfIdEnable = 1,
    IdExClear = 2,
}
pub enum Connect {
    LoadSignal = 0,
    ExRd = 1,
    IdRs1 = 2,
    IdRs2 = 3,
    NpcSel = 4,
}
pub struct HazardBuilder {
    pub pc_enable: ControlShared<PcEnable>,
    pub if_id_enable: ControlShared<IfIdEnable>,
    pub id_ex_clear: ControlShared<IdExClear>,
    pub idex_clr: RegBuilder,
    pub pc_en: RegBuilder,
    pub ifid_en: RegBuilder,
}
impl HazardBuilder {
    pub fn new() -> Self {
        let mut pc_enable = ControlShared::new(PcEnable::default());
        let mut if_id_enable = ControlShared::new(IfIdEnable::default());
        let mut id_ex_clear = ControlShared::new(IdExClear::default());
        let mut idex_clr = RegBuilder::new(0);
        let mut pc_en = RegBuilder::new(1);
        let mut ifid_en = RegBuilder::new(1);
        let mut consts = ConstsBuilder::default();
        consts.push(0);
        consts.push(1);
        idex_clr.connect(pc_enable.clone().into_shared().into(), RegConnect::In);
        idex_clr.connect(consts.alloc(ConstsAlloc::Out(0)), RegConnect::Clear);
        idex_clr.connect(consts.alloc(ConstsAlloc::Out(1)), RegConnect::Enable);
        ifid_en.connect(if_id_enable.clone().into_shared().into(), RegConnect::In);
        ifid_en.connect(consts.alloc(ConstsAlloc::Out(0)), RegConnect::Clear);
        ifid_en.connect(consts.alloc(ConstsAlloc::Out(1)), RegConnect::Enable);
        pc_en.connect(pc_enable.clone().into_shared().into(), RegConnect::In);
        pc_en.connect(consts.alloc(ConstsAlloc::Out(0)), RegConnect::Clear);
        pc_en.connect(consts.alloc(ConstsAlloc::Out(1)), RegConnect::Enable);
        HazardBuilder {
            pc_enable,
            if_id_enable,
            id_ex_clear,
            idex_clr,
            pc_en,
            ifid_en,
        }
    }
}
impl ControlBuilder for HazardBuilder {
    fn build(self) -> ControlRef {
        Hazard {
            pc_enable: self.pc_enable.into_shared().into(),
            if_id_enable: self.if_id_enable.into_shared().into(),
            id_ex_clear: self.id_ex_clear.into_shared().into(),
            clr: self.idex_clr.build(),
            pc_en: self.pc_en.build(),
            ifid_en: self.ifid_en.build(),
        }
        .into()
    }
}
impl PortBuilder for HazardBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::PcEnable => self.pc_en.alloc(RegAlloc::Out),
            Alloc::IfIdEnable => self.ifid_en.alloc(RegAlloc::Out),
            Alloc::IdExClear => self.idex_clr.alloc(RegAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::LoadSignal => {
                self.pc_enable.borrow_mut().load_signal = Some(pin.clone());
                self.if_id_enable.borrow_mut().load_signal = Some(pin.clone());
                self.id_ex_clear.borrow_mut().load_signal = Some(pin.clone());
            }
            Connect::ExRd => {
                self.pc_enable.borrow_mut().ex_rd = Some(pin.clone());
                self.if_id_enable.borrow_mut().ex_rd = Some(pin.clone());
                self.id_ex_clear.borrow_mut().ex_rd = Some(pin.clone());
            }
            Connect::IdRs1 => {
                self.pc_enable.borrow_mut().id_rs1 = Some(pin.clone());
                self.if_id_enable.borrow_mut().id_rs1 = Some(pin.clone());
                self.id_ex_clear.borrow_mut().id_rs1 = Some(pin.clone());
            }
            Connect::IdRs2 => {
                self.pc_enable.borrow_mut().id_rs2 = Some(pin.clone());
                self.if_id_enable.borrow_mut().id_rs2 = Some(pin.clone());
                self.id_ex_clear.borrow_mut().id_rs2 = Some(pin.clone());
            }
            Connect::NpcSel => {
                self.id_ex_clear.borrow_mut().npc_sel = Some(pin.clone());
            }
        }
    }
}
#[derive(Debug)]
pub struct Hazard {
    pub pc_enable: ControlRef,
    pub if_id_enable: ControlRef,
    pub id_ex_clear: ControlRef,
    pub clr: ControlRef,
    pub pc_en: ControlRef,
    pub ifid_en: ControlRef,
}
impl Control for Hazard {
    fn rasing_edge(&mut self) {
        self.clr.rasing_edge();
        self.pc_en.rasing_edge();
        self.ifid_en.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.pc_enable.falling_edge();
        self.if_id_enable.falling_edge();
        self.id_ex_clear.falling_edge();
    }
    fn output(&self) -> Vec<(String, u32)> {
        let mut output = Vec::new();
        output.extend(self.pc_enable.output());
        output.extend(self.if_id_enable.output());
        output.extend(self.id_ex_clear.output());
        output
    }
}
#[derive(Default, Debug)]
pub struct PcEnable {
    pub load_signal: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}

impl Control for PcEnable {
    fn output(&self) -> Vec<(String, u32)> {
        vec![("pc_en".to_string(), self.read())]
    }
}

impl Port for PcEnable {
    fn read(&self) -> u32 {
        let load_signal = match self.load_signal {
            Some(ref load_signal) => load_signal.read(),
            None => {
                unimplemented!()
            }
        };
        let ex_rd = match self.ex_rd {
            Some(ref ex_rd) => ex_rd.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs1 = match self.id_rs1 {
            Some(ref id_rs1) => id_rs1.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs2 = match self.id_rs2 {
            Some(ref id_rs2) => id_rs2.read(),
            None => {
                unimplemented!()
            }
        };
        if load_signal == 1 && (ex_rd == id_rs1 || ex_rd == id_rs2) {
            0
        } else {
            1
        }
    }
}

#[derive(Default, Debug)]
pub struct IfIdEnable {
    pub load_signal: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}
impl Control for IfIdEnable {
    fn output(&self) -> Vec<(String, u32)> {
        vec![("ifid_en".to_string(), self.read())]
    }
}
impl Port for IfIdEnable {
    fn read(&self) -> u32 {
        let load_signal = match self.load_signal {
            Some(ref load_signal) => load_signal.read(),
            None => {
                unimplemented!()
            }
        };
        let ex_rd = match self.ex_rd {
            Some(ref ex_rd) => ex_rd.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs1 = match self.id_rs1 {
            Some(ref id_rs1) => id_rs1.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs2 = match self.id_rs2 {
            Some(ref id_rs2) => id_rs2.read(),
            None => {
                unimplemented!()
            }
        };
        if load_signal == 1 && (ex_rd == id_rs1 || ex_rd == id_rs2) {
            0
        } else {
            1
        }
    }
}

#[derive(Default, Debug)]
pub struct IdExClear {
    pub load_signal: Option<PortRef>,
    pub npc_sel: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}
impl Control for IdExClear {
    fn output(&self) -> Vec<(String, u32)> {
        vec![("idex_clr".to_string(), self.read())]
    }
}
impl Port for IdExClear {
    fn read(&self) -> u32 {
        let load_signal = match self.load_signal {
            Some(ref load_signal) => load_signal.read(),
            None => {
                unimplemented!()
            }
        };
        let npc_sel = match self.npc_sel {
            Some(ref npc_sel) => npc_sel.read(),
            None => {
                unimplemented!()
            }
        };
        let ex_rd = match self.ex_rd {
            Some(ref ex_rd) => ex_rd.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs1 = match self.id_rs1 {
            Some(ref id_rs1) => id_rs1.read(),
            None => {
                unimplemented!()
            }
        };
        let id_rs2 = match self.id_rs2 {
            Some(ref id_rs2) => id_rs2.read(),
            None => {
                unimplemented!()
            }
        };
        if load_signal == 1 && (ex_rd == id_rs1 || ex_rd == id_rs2) {
            1
        } else if npc_sel == 1 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;
    struct TestAlloc {
        pub pc_enable: u32,
        pub if_id_enable: u32,
        pub id_ex_clear: u32,
    }
    struct TestConnect {
        pub load_signal: u32,
        pub ex_rd: u32,
        pub id_rs1: u32,
        pub id_rs2: u32,
        pub npc_sel: u32,
    }
    fn run_test(alloc: TestAlloc, connect: TestConnect) {
        let mut builder = HazardBuilder::new();
        let pc_enable = builder.alloc(Alloc::PcEnable);
        let if_id_enable = builder.alloc(Alloc::IfIdEnable);
        let id_ex_clear = builder.alloc(Alloc::IdExClear);
        let mut consts = ConstsBuilder::default();
        consts.push(connect.load_signal);
        consts.push(connect.ex_rd);
        consts.push(connect.id_rs1);
        consts.push(connect.id_rs2);
        consts.push(connect.npc_sel);
        builder.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::LoadSignal);
        builder.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::ExRd);
        builder.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::IdRs1);
        builder.connect(consts.alloc(ConstsAlloc::Out(3)), Connect::IdRs2);
        builder.connect(consts.alloc(ConstsAlloc::Out(4)), Connect::NpcSel);
        assert_eq!(pc_enable.read(), alloc.pc_enable);
        assert_eq!(if_id_enable.read(), alloc.if_id_enable);
        assert_eq!(id_ex_clear.read(), alloc.id_ex_clear);
    }
    #[test]
    fn test_hazard0() {
        run_test(
            TestAlloc {
                pc_enable: 1,
                if_id_enable: 1,
                id_ex_clear: 0,
            },
            TestConnect {
                load_signal: 0,
                ex_rd: 0xf,
                id_rs1: 8,
                id_rs2: 0xf,
                npc_sel: 0,
            },
        );
    }
    #[test]
    fn test_hazard1() {
        run_test(
            TestAlloc {
                pc_enable: 0,
                if_id_enable: 0,
                id_ex_clear: 1,
            },
            TestConnect {
                load_signal: 1,
                ex_rd: 0xf,
                id_rs1: 0xf,
                id_rs2: 1,
                npc_sel: 0,
            },
        );
    }
    #[test]
    fn test_hazard2() {
        run_test(
            TestAlloc {
                pc_enable: 1,
                if_id_enable: 1,
                id_ex_clear: 1,
            },
            TestConnect {
                load_signal: 0,
                ex_rd: 0x15,
                id_rs1: 0,
                id_rs2: 1,
                npc_sel: 1,
            },
        );
    }
}
