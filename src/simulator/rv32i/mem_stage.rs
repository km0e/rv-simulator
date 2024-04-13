use crate::component::{build::MemBuilder, Builder, PortRef};
pub enum Alloc {
    Out = 0,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Out => 0,
        }
    }
}
pub enum Connect {
    Address = 0,
    Data = 1,
    Write = 2,
    Read = 3,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Address => 0,
            Connect::Data => 1,
            Connect::Write => 2,
            Connect::Read => 3,
        }
    }
}
#[derive(Default)]
pub struct MemStageBuilder {
    pub inner: MemBuilder,
}

impl Builder for MemStageBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        self.inner.alloc(id)
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        self.inner.connect(pin, id)
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        self.inner.build()
    }
}
