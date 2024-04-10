use super::bomb::Bomb;
use super::Builder;
use super::Component;
use super::ComponentRef;
use super::ComponentShared;
use super::ControlRef;
#[derive(Default)]
pub struct MuxBuilder {
    pub inner: ComponentShared<Mux>,
}
impl Builder for MuxBuilder {
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().select = Some(pin),
            c => {
                let input = &mut self.inner.borrow_mut().input;
                if c >= input.len() {
                    input.resize(c, ComponentRef::from(ComponentShared::new(Bomb::default())));
                }
                input[c - 1] = pin;
            }
        }
    }
    fn alloc(&mut self, _: usize) -> ComponentRef {
        ComponentRef::from(self.inner.clone())
    }
    fn build(self) -> Option<ControlRef> {
        None
    }
}
#[derive(Default)]
pub struct Mux {
    pub input: Vec<ComponentRef>,
    pub select: Option<ComponentRef>, // select input
}
impl Component for Mux {
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
