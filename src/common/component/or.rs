use crate::common::abi::*;
pub enum Alloc {
    Out,
}
pub enum Connect {
    In,
}
#[derive(Default)]
pub struct OrBuilder {
    inner: PortShared<Or>,
}
impl ControlBuilder for OrBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for OrBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, _: Self::Connect) {
        self.inner.borrow_mut().input.push(pin);
    }
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Or {
    pub mask: u32,
    pub input: Vec<PortRef>,
}
impl Default for Or {
    fn default() -> Self {
        Self {
            mask: 0x1,
            input: Vec::new(),
        }
    }
}
impl Control for Or {
    fn rasing_edge(&mut self) {}
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("or", self.read())]
    }
}
impl Port for Or {
    fn read(&self) -> u32 {
        self.input
            .iter()
            .map(|x| x.read())
            .fold(0, |acc, x| acc | x)
            & self.mask
    }
}

pub mod build {
    pub use super::Alloc as OrAlloc;
    pub use super::Connect as OrConnect;
    pub use super::OrBuilder;
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_add() {
        let mut tb = OrBuilder::default();
        let mut constant = ConstsBuilder::default();
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), Connect::In);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::In);
        let t = tb.alloc(Alloc::Out);
        assert_eq!(t.read(), 1);
    }
}
