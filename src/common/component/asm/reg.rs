use crate::common::abi::*;
use crate::common::build::*;

use super::{AsmBuilder, AsmPort, AsmPortRef, AsmPortShared};
pub enum AsmRegAlloc {
    Out,
}
impl From<AsmRegAlloc> for usize {
    fn from(alloc: AsmRegAlloc) -> usize {
        match alloc {
            AsmRegAlloc::Out => 0,
        }
    }
}
pub enum AsmRegConnect {
    In,
}
impl From<AsmRegConnect> for usize {
    fn from(alloc: AsmRegConnect) -> usize {
        match alloc {
            AsmRegConnect::In => 0,
        }
    }
}

#[derive(Default)]
pub struct AsmRegBuilder {
    pub inner: Option<ControlShared<AsmReg>>,
}
impl ControlBuilder for AsmRegBuilder {
    fn build(self) -> Box<dyn Control> {
        self.inner.unwrap().into()
    }
}
impl AsmBuilder for AsmRegBuilder {
    fn asm_alloc(&self, id: usize) -> AsmPortRef {
        assert_eq!(
            id,
            AsmRegAlloc::Out.into(),
            "AsmRegBuilder: invalid asm alloc id"
        );
        if let Some(inner) = &self.inner {
            inner.clone().into()
        } else {
            panic!("AsmRegBuilder: asm alloc before asm connect")
        }
    }
    fn asm_connect(&mut self, pin: AsmPortRef, id: usize) {
        assert_eq!(
            id,
            AsmRegConnect::In.into(),
            "AsmRegBuilder: invalid asm connect id"
        );
        self.inner = Some(ControlShared::new(AsmReg::new(pin)));
    }
}
pub struct AsmReg {
    pub prev: AsmPortRef,
    pub tmp: String,
    pub inst: String,
}
impl AsmReg {
    pub fn new(ap: impl Into<AsmPortRef>) -> Self {
        Self {
            prev: ap.into(),
            tmp: "".to_string(),
            inst: "".to_string(),
        }
    }
}
impl Control for AsmReg {
    fn rasing_edge(&mut self) {
        self.tmp = self.prev.read();
    }
    fn falling_edge(&mut self) {
        self.inst = self.tmp.clone();
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        self.inst.clone()
    }
}

impl AsmPort for AsmReg {
    fn read(&self) -> String {
        self.inst.clone()
    }
}
