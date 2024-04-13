use super::bomb::Bomb;
use super::Builder;
use super::ControlRef;
use super::Port;
use super::PortRef;
use super::PortShared;
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
    Select,
    In(usize),
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Select => 0,
            Connect::In(c) => c + 1,
        }
    }
}
#[derive(Default)]
pub struct MuxBuilder {
    pub inner: PortShared<Mux>,
}
impl Builder for MuxBuilder {
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().select = Some(pin),
            c => {
                let input = &mut self.inner.borrow_mut().input;
                if c > input.len() {
                    input.resize(c, PortRef::from(PortShared::new(Bomb::default())));
                }
                input[c - 1] = pin;
            }
        }
    }
    fn alloc(&mut self, _: usize) -> PortRef {
        PortRef::from(self.inner.clone())
    }
    fn build(self) -> Option<ControlRef> {
        None
    }
}
#[derive(Default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::consts::ConstsBuilder;

    #[test]
    fn test_mux() {
        let mut tb = MuxBuilder::default();
        let mut t = tb.alloc(0);
        let mut constant = ConstsBuilder::default();
        constant.push(1);
        constant.push(2);
        constant.push(0);
        tb.connect(constant.alloc(2), 0);
        tb.connect(constant.alloc(0), 1);
        tb.connect(constant.alloc(1), 2);
        assert_eq!(t.read(), 1);
        assert_eq!(t.read(), 1);
        assert_eq!(t.read(), 1);
    }
}
