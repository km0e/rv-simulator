use super::lat::Lat;
use super::{Builder, ComponentRef, ComponentShared, Control, ControlRef, ControlShared};

pub struct RegBuilder {
    inner: ControlShared<Reg>,
}
impl RegBuilder {
    pub fn new(value: u32) -> Self {
        Self {
            inner: ControlShared::new(Reg {
                output: ComponentShared::new(Lat::new(value)),
                ..Default::default()
            }),
        }
    }
}
impl Builder for RegBuilder {
    fn connect(&mut self, pin: ComponentRef, _: usize) {
        self.inner.borrow_mut().input = Some(pin);
    }
    fn alloc(&mut self, _: usize) -> ComponentRef {
        ComponentRef::from(self.inner.borrow().output.clone())
    }
    fn build(self) -> Option<ControlRef> {
        Some(ControlRef::from(self.inner.clone()))
    }
}

#[derive(Default)]
pub struct Reg {
    pub input: Option<ComponentRef>,
    pub data: u32,
    pub output: ComponentShared<Lat>,
}
impl Control for Reg {
    fn rasing_edge(&mut self) {
        if let Some(input) = self.input.as_ref() {
            self.data = input.read();
        }
    }
    fn falling_edge(&mut self) {
        self.output.borrow_mut().data = self.data;
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;
    use crate::component::consts::ConstsBuilder;

    #[test]
    fn test_reg() {
        let mut tb = RegBuilder::new(1);
        let mut constant = ConstsBuilder::default();
        constant.push(2);
        tb.connect(constant.alloc(0), 0);
        let t = tb.alloc(0);
        let mut tc = tb.build().unwrap();
        assert_eq!(t.read(), 1);
        tc.borrow_mut().rasing_edge();
        assert_eq!(t.read(), 1);
        tc.borrow_mut().falling_edge();
        assert_eq!(t.read(), 2);
    }
}
