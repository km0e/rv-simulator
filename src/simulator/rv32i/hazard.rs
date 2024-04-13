use crate::{
    component::{
        Builder, Composite, CompositeRef, CompositeShared, ControlRef, ControlShared, Port,
        PortRef, PortShared,
    },
    Control,
};
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
    pub pc_enable: CompositeShared<PcEnable>,
    pub if_id_enable: CompositeShared<IfIdEnable>,
    pub id_ex_clear: CompositeShared<IdExClear>,
}
impl Builder for HazardBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => PortRef::from(self.pc_enable.clone()),
            1 => PortRef::from(self.if_id_enable.clone()),
            2 => PortRef::from(self.id_ex_clear.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => {
                self.pc_enable.borrow_mut().load_signal = Some(pin.clone());
                self.if_id_enable.borrow_mut().load_signal = Some(pin.clone());
                self.id_ex_clear.borrow_mut().load_signal = Some(pin.clone());
            }
            1 => {
                self.pc_enable.borrow_mut().ex_rd = Some(pin.clone());
                self.if_id_enable.borrow_mut().ex_rd = Some(pin.clone());
                self.id_ex_clear.borrow_mut().ex_rd = Some(pin.clone());
            }
            2 => {
                self.pc_enable.borrow_mut().id_rs1 = Some(pin.clone());
                self.if_id_enable.borrow_mut().id_rs1 = Some(pin.clone());
                self.id_ex_clear.borrow_mut().id_rs1 = Some(pin.clone());
            }
            3 => {
                self.pc_enable.borrow_mut().id_rs2 = Some(pin.clone());
                self.if_id_enable.borrow_mut().id_rs2 = Some(pin.clone());
                self.id_ex_clear.borrow_mut().id_rs2 = Some(pin.clone());
            }
            4 => {
                self.id_ex_clear.borrow_mut().npc_sel = Some(pin.clone());
            }
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        Some(
            ControlShared::new(Hazard {
                pc_enable: CompositeRef::from(self.pc_enable),
                if_id_enable: CompositeRef::from(self.if_id_enable),
                id_ex_clear: CompositeRef::from(self.id_ex_clear),
            })
            .into(),
        )
    }
}
pub struct Hazard {
    pub pc_enable: CompositeRef,
    pub if_id_enable: CompositeRef,
    pub id_ex_clear: CompositeRef,
}
impl Control for Hazard {
    fn debug(&self) -> String {
        format!(
            "Hazard\nPC_EN\t\t: {:8}IF_IF_EN\t: {:8}ID_EX_CLR\t: {:8}",
            self.pc_enable.debug(),
            self.if_id_enable.debug(),
            self.id_ex_clear.debug(),
        )
    }
}
#[derive(Default)]
pub struct PcEnable {
    pub load_signal: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}

impl Control for PcEnable {
    fn debug(&self) -> String {
        format!("{:X}", self.read())
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
impl Composite for PcEnable {}

#[derive(Default)]
pub struct IfIdEnable {
    pub load_signal: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}
impl Control for IfIdEnable {
    fn debug(&self) -> String {
        format!("{:X}", self.read())
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
impl Composite for IfIdEnable {}

#[derive(Default)]
pub struct IdExClear {
    pub load_signal: Option<PortRef>,
    pub npc_sel: Option<PortRef>,
    pub ex_rd: Option<PortRef>,
    pub id_rs1: Option<PortRef>,
    pub id_rs2: Option<PortRef>,
}
impl Control for IdExClear {
    fn debug(&self) -> String {
        format!("{:X}", self.read())
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
impl Composite for IdExClear {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::*;
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
        let pc_enable = builder.alloc(0);
        let if_id_enable = builder.alloc(1);
        let id_ex_clear = builder.alloc(2);
        let mut consts = ConstsBuilder::default();
        consts.push(connect.load_signal);
        consts.push(connect.ex_rd);
        consts.push(connect.id_rs1);
        consts.push(connect.id_rs2);
        consts.push(connect.npc_sel);
        builder.connect(consts.alloc(0), Connect::LoadSignal.into());
        builder.connect(consts.alloc(1), Connect::ExRd.into());
        builder.connect(consts.alloc(2), Connect::IdRs1.into());
        builder.connect(consts.alloc(3), Connect::IdRs2.into());
        builder.connect(consts.alloc(4), Connect::NpcSel.into());
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
