use crate::common::abi::*;

pub enum Alloc {
    PcEnable = 0,
    IfIdEnable = 1,
    IdExClear = 2,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::PcEnable => 0,
            Alloc::IfIdEnable => 1,
            Alloc::IdExClear => 2,
        }
    }
}
pub enum Connect {
    LoadSignal = 0,
    ExRd = 1,
    IdRs1 = 2,
    IdRs2 = 3,
    NpcSel = 4,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::LoadSignal => 0,
            Connect::ExRd => 1,
            Connect::IdRs1 => 2,
            Connect::IdRs2 => 3,
            Connect::NpcSel => 4,
        }
    }
}
#[derive(Default)]
pub struct HazardBuilder {
    pub pc_enable: PortShared<PcEnable>,
    pub if_id_enable: PortShared<IfIdEnable>,
    pub id_ex_clear: PortShared<IdExClear>,
}
impl ControlBuilder for HazardBuilder {
    fn build(self) -> ControlRef {
        Hazard {
            pc_enable: self.pc_enable.into_shared().into(),
            if_id_enable: self.if_id_enable.into_shared().into(),
            id_ex_clear: self.id_ex_clear.into_shared().into(),
        }
        .into()
    }
}
impl PortBuilder for HazardBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::PcEnable => PortRef::from(self.pc_enable.clone()),
            Alloc::IfIdEnable => PortRef::from(self.if_id_enable.clone()),
            Alloc::IdExClear => PortRef::from(self.id_ex_clear.clone()),
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
}
impl Control for Hazard {}
#[derive(Default, Debug)]
pub struct PcEnable {
    pub load_signal: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}

impl Control for PcEnable {}

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
impl Control for IfIdEnable {}
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
impl Control for IdExClear {}
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
        let mut builder = HazardBuilder::default();
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
