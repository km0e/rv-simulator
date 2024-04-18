use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out = 0,
}

pub enum Connect {
    In,
    Enable,
    Clear,
}
#[derive(Default)]
pub struct RegBuilder {
    inner: ControlShared<Reg>,
}
impl RegBuilder {
    pub fn new(value: u32) -> Self {
        Self {
            inner: ControlShared::new(Reg {
                output: PortShared::new(Lat::new(value)),
                ..Default::default()
            }),
        }
    }
}
impl ControlBuilder for RegBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for RegBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::In => self.inner.borrow_mut().input = pin,
            Self::Connect::Enable => self.inner.borrow_mut().enable = pin,
            Self::Connect::Clear => self.inner.borrow_mut().clear = pin,
        }
    }
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.borrow().output.clone())
    }
}

#[derive(Debug)]
pub struct Reg {
    pub input: PortRef,
    pub enable: PortRef,
    pub en_cache: u32,
    pub clear: PortRef,
    pub clr_cache: u32,
    pub data: u32,
    pub output: PortShared<Lat>,
}
impl Reg {
    pub fn new(value: u32) -> Self {
        Self {
            output: PortShared::new(Lat::new(value)),
            en_cache: 1,
            clr_cache: 0,
            input: bomb().into(),
            enable: bomb().into(),
            clear: bomb().into(),
            data: value,
        }
    }
}
impl Default for Reg {
    fn default() -> Self {
        Self::new(0)
    }
}
impl Control for Reg {
    fn rasing_edge(&mut self) {
        if self.clear.read() == 1 {
            self.clr_cache = 1;
            return;
        }
        if self.enable.read() == 0 {
            self.en_cache = 0;
            return;
        }
        self.data = self.input.read();
    }
    fn falling_edge(&mut self) {
        if self.clr_cache == 1 {
            self.output.borrow_mut().data = 0;
            self.clr_cache = 0;
            return;
        }
        if self.en_cache == 0 {
            self.en_cache = 1;
            return;
        }
        self.output.borrow_mut().data = self.data;
    }
    fn input(&self) -> Vec<(&'static str, u32)> {
        vec![
            ("in", self.input.read()),
            ("en", self.enable.read()),
            ("clr", self.clear.read()),
        ]
    }
    fn output(&self) -> Vec<(&'static str, u32)> {
        vec![("out", self.output.borrow().data)]
    }
}
pub mod build {
    pub use super::Alloc as RegAlloc;
    pub use super::Connect as RegConnect;
    pub use super::RegBuilder;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg() {
        let mut tb = RegBuilder::new(1);
        let mut constant = ConstsBuilder::default();
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), Connect::In);
        let t = tb.alloc(RegAlloc::Out);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), Connect::Enable);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::Clear);
        let tc = tb.build();
        assert_eq!(t.read(), 1);
        tc.rasing_edge();
        assert_eq!(t.read(), 1);
        tc.falling_edge();
        assert_eq!(t.read(), 2);
    }
}
