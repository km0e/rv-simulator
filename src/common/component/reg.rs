use crate::common::abi::*;
use crate::common::build::*;
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
    In,
    Enable,
    Clear,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::In => 0,
            Connect::Enable => 1,
            Connect::Clear => 2,
        }
    }
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
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().in_ = Some(pin),
            1 => self.inner.borrow_mut().enable = Some(pin),
            2 => self.inner.borrow_mut().clear = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
    fn alloc(&mut self, _: usize) -> PortRef {
        PortRef::from(self.inner.borrow().output.clone())
    }
}

#[derive(Default)]
pub struct Reg {
    pub in_: Option<PortRef>,
    pub enable: Option<PortRef>,
    pub clear: Option<PortRef>,
    pub data: u32,
    pub output: PortShared<Lat>,
}
impl Control for Reg {
    fn rasing_edge(&mut self) {
        if self
            .enable
            .as_ref()
            .expect("reg enable not connected")
            .read()
            == 0
        {
            return;
        }
        if let Some(in_) = self.in_.as_ref() {
            self.data = in_.read();
        }
    }
    fn falling_edge(&mut self) {
        if let Some(clear) = self.clear.as_ref() {
            if clear.read() == 1 {
                self.output.borrow_mut().data = 0;
            }
        }
        if let Some(enable) = self.enable.as_ref() {
            if enable.read() == 0 {
                return;
            }
        }
        self.output.borrow_mut().data = self.data;
    }
    #[cfg(debug_assertions)]
    fn debug(&self) -> String {
        format!("{:X}", self.output.borrow().data)
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
        constant.push(2);
        constant.push(1);
        constant.push(1);
        tb.connect(constant.alloc(0), Connect::In.into());
        let t = tb.alloc(0);
        tb.connect(constant.alloc(1), Connect::Enable.into());
        tb.connect(constant.alloc(2), Connect::Clear.into());
        let tc = tb.build();
        assert_eq!(t.read(), 1);
        tc.rasing_edge();
        assert_eq!(t.read(), 1);
        tc.falling_edge();
        assert_eq!(t.read(), 2);
    }
}
