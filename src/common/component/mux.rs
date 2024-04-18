use std::collections::HashMap;

use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    Select,
    In(u32),
}

#[derive(Debug)]
pub struct MuxBuilder {
    inner: PortShared<Mux>,
}
impl Default for MuxBuilder {
    fn default() -> Self {
        Self {
            inner: PortShared::new(Mux {
                input: HashMap::new(),
                select: bomb().into(),
            }),
        }
    }
}
impl ControlBuilder for MuxBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for MuxBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Select => self.inner.borrow_mut().select = pin,
            Self::Connect::In(id) => {
                self.inner.borrow_mut().input.insert(id, pin);
            }
        };
    }
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
}
#[derive(Debug, Clone)]
pub struct Mux {
    input: HashMap<u32, PortRef>, // input
    select: PortRef,              // select input
}

impl Port for Mux {
    fn read(&self) -> u32 {
        let id = self.select.read();
        self.input.get(&id).expect("mux input").read()
    }
}
impl Control for Mux {
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("mux", self.read())]
    }
}
pub mod build {
    pub use super::Alloc as MuxAlloc;
    pub use super::Connect as MuxConnect;
    pub use super::MuxBuilder;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux() {
        let mut tb = MuxBuilder::default();
        let mut constant = ConstsBuilder::default();
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MuxConnect::Select);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MuxConnect::In(0));
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MuxConnect::In(1));
        let t = tb.alloc(MuxAlloc::Out);
        let _ = tb.build();
        assert_eq!(t.read(), 1);
    }
}
