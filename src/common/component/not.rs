use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out,
}
pub enum Connect {
    In,
}
#[derive(Default)]
pub struct NotBuilder {
    inner: PortShared<Not>,
}
impl ControlBuilder for NotBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for NotBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, _: Self::Connect) {
        self.inner.borrow_mut().input = pin;
    }
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Not {
    pub mask: u32,
    pub input: PortRef,
}
impl Default for Not {
    fn default() -> Self {
        Self {
            mask: 0x1,
            input: bomb().into(),
        }
    }
}
impl Control for Not {
    fn rasing_edge(&mut self) {}
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("not", self.read())]
    }
}
impl Port for Not {
    fn read(&self) -> u32 {
        let data = self.input.read();
        (!data) & self.mask
    }
}

pub mod build {
    pub use super::Alloc as NotAlloc;
    pub use super::Connect as NotConnect;
    pub use super::NotBuilder;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut tb = NotBuilder::default();
        let mut constant = ConstsBuilder::default();
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), Connect::In);
        let t = tb.alloc(Alloc::Out);
        assert_eq!(t.read(), 0);
    }
}
