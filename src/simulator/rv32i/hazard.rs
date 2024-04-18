use crate::common::abi::*;
use crate::common::build::*;
mod raw;
use raw::build::*;
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
    pub raw: RAWBuilder,
    pub not: NotBuilder,
    pub or: OrBuilder,
}
impl Default for HazardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HazardBuilder {
    pub fn new() -> Self {
        let mut raw = RAWBuilder::default();
        let mut not = NotBuilder::default();
        not.connect(raw.alloc(RAWAlloc::Out), NotConnect::In);
        let mut or = OrBuilder::default();
        or.connect(raw.alloc(RAWAlloc::Out), OrConnect::In);
        HazardBuilder { not, raw, or }
    }
}
impl ControlBuilder for HazardBuilder {
    fn build(self) -> ControlRef {
        Hazard {
            raw: self.raw.build(),
            not: self.not.build(),
            or: self.or.build(),
        }
        .into()
    }
}
impl PortBuilder for HazardBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::PcEnable => self.not.alloc(NotAlloc::Out),
            Alloc::IfIdEnable => self.not.alloc(NotAlloc::Out),
            Alloc::IdExClear => self.or.alloc(OrAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::LoadSignal => {
                self.raw.connect(pin.clone(), RAWConnect::En);
            }
            Connect::ExRd => {
                self.raw.connect(pin.clone(), RAWConnect::Rd);
            }
            Connect::IdRs1 => {
                self.raw.connect(pin.clone(), RAWConnect::Rs1);
            }
            Connect::IdRs2 => {
                self.raw.connect(pin.clone(), RAWConnect::Rs2);
            }
            Connect::NpcSel => {
                self.or.connect(pin.clone(), OrConnect::In);
            }
        }
    }
}
#[derive(Debug)]
pub struct Hazard {
    pub raw: ControlRef,
    pub not: ControlRef,
    pub or: ControlRef,
}
impl Control for Hazard {
    fn rasing_edge(&mut self) {
        self.raw.rasing_edge();
        self.not.rasing_edge();
        self.or.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.raw.falling_edge();
        self.not.falling_edge();
        self.or.falling_edge();
    }
    fn output(&self) -> Vec<(&'static str, u32)> {
        let mut res = vec![("en", self.not.output()[0].1)];
        res.extend(self.raw.output());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        builder.connect(
            consts.alloc(ConstsAlloc::Out(connect.load_signal)),
            Connect::LoadSignal,
        );
        builder.connect(consts.alloc(ConstsAlloc::Out(connect.ex_rd)), Connect::ExRd);
        builder.connect(
            consts.alloc(ConstsAlloc::Out(connect.id_rs1)),
            Connect::IdRs1,
        );
        builder.connect(
            consts.alloc(ConstsAlloc::Out(connect.id_rs2)),
            Connect::IdRs2,
        );
        builder.connect(
            consts.alloc(ConstsAlloc::Out(connect.npc_sel)),
            Connect::NpcSel,
        );
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
