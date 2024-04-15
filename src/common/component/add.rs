use crate::common::abi::*;
pub enum Alloc {
    Out,
}
pub enum Connect {
    In(usize),
}
#[derive(Default)]
pub struct AddBuilder {
    inner: PortShared<Add>,
}
impl AddBuilder {
    pub fn new() -> Self {
        Self {
            inner: PortShared::new(Add::default()),
        }
    }
}

impl PortBuilder for AddBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, _: Self::Connect) {
        self.inner.borrow_mut().input.push(pin);
    }
    fn alloc(&mut self, _id: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
}

#[derive(Default, Debug)]
pub struct Add {
    pub input: Vec<PortRef>,
}

impl Port for Add {
    fn read(&self) -> u32 {
        self.input.iter().map(|x| x.read()).sum()
    }
}

pub mod build {
    pub use super::AddBuilder;
    pub use super::Alloc as AddAlloc;
    pub use super::Connect as AddConnect;
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_add() {
        let mut tb = AddBuilder::default();
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        let t = tb.alloc(Alloc::Out);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::In(0));
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), Connect::In(1));
        assert_eq!(t.read(), 3);
        assert_eq!(t.read(), 3);
    }
}
