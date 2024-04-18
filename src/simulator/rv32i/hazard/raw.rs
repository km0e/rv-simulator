use std::cell::Cell;

use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    Rs1 = 0,
    Rs2 = 1,
    En = 2,
    Rd = 3,
}
#[derive(Default)]
pub struct RAWBuilder {
    inner: PortShared<RAW>,
}
impl ControlBuilder for RAWBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for RAWBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, _id: Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Rs1 => self.inner.borrow_mut().rs1 = pin,
            Connect::Rs2 => self.inner.borrow_mut().rs2 = pin,
            Connect::En => self.inner.borrow_mut().en = pin,
            Connect::Rd => self.inner.borrow_mut().rd = pin,
        }
    }
}
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct RAW {
    pub mask: u32,
    pub rs1: PortRef,
    pub rs2: PortRef,
    pub en: PortRef,
    pub rd: PortRef,
    pub out_cache: Cell<Option<u32>>,
}
impl Default for RAW {
    fn default() -> Self {
        Self {
            mask: 0x1,
            rs1: bomb().into(),
            rs2: bomb().into(),
            en: bomb().into(),
            rd: bomb().into(),
            out_cache: Cell::new(None),
        }
    }
}
impl Control for RAW {
    fn rasing_edge(&mut self) {
        self.out_cache = Cell::new(None);
    }
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("raw", self.read())]
    }
}
impl Port for RAW {
    fn read(&self) -> u32 {
        let v = match self.out_cache.get() {
            Some(value) => value,
            None => {
                let value = if self.en.read() == 1
                    && (self.rs1.read() == self.rd.read() || self.rs2.read() == self.rd.read())
                {
                    1
                } else {
                    0
                };
                self.out_cache.replace(Some(value));
                value
            }
        };
        v & self.mask
    }
}

pub mod build {
    pub use super::Alloc as RAWAlloc;
    pub use super::Connect as RAWConnect;
    pub use super::RAWBuilder;
}
