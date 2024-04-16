use crate::common::abi::*;
use crate::common::build::*;
pub enum Alloc {
    Out = 0,
}
pub enum Connect {
    Select,
    In(usize),
}
#[derive(Default)]
pub struct MuxBuilder {
    pub inner: PortShared<Mux>,
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
            Self::Connect::Select => self.inner.borrow_mut().select = Some(pin),
            Self::Connect::In(c) => {
                let input = &mut self.inner.borrow_mut().input;
                if c == input.len() {
                    input.resize(c + 1, Bomb::default().into());
                }
                input[c] = pin;
            }
        }
    }
    fn alloc(&mut self, _: Self::Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
}
#[derive(Debug, Default)]
pub struct Mux {
    pub input: Vec<PortRef>,
    pub select: Option<PortRef>, // select input
}
impl Port for Mux {
    fn read(&self) -> u32 {
        let id = self.select.as_ref().unwrap().read();
        self.input[id as usize].read()
    }
}
impl Control for Mux {
    fn output(&self) -> Vec<(String, u32)> {
        vec![("mux".to_string(), self.read())]
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
        let t = tb.alloc(MuxAlloc::Out);
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        constant.push(0);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), MuxConnect::Select);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), MuxConnect::In(0));
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), MuxConnect::In(1));
        assert_eq!(t.read(), 1);
        assert_eq!(t.read(), 1);
        assert_eq!(t.read(), 1);
    }
}
