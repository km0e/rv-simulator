use super::utils::u2i;
use crate::component::{Builder, Port, PortRef, PortShared};
pub enum Alloc {
    Res = 0,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Res => 0,
        }
    }
}
pub enum Connect {
    Ctrl = 0,
    Op1 = 1,
    Op2 = 2,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Ctrl => 0,
            Connect::Op1 => 1,
            Connect::Op2 => 2,
        }
    }
}
#[derive(Default)]
pub struct AluBuilder {
    inner: PortShared<Alu>,
}
impl Builder for AluBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        match id {
            0 => PortRef::from(self.inner.clone()),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        None
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().alu_ctl = Some(pin.clone()),
            1 => self.inner.borrow_mut().input1 = Some(pin.clone()),
            2 => self.inner.borrow_mut().input2 = Some(pin.clone()),
            _ => panic!("Invalid id"),
        }
    }
}

#[derive(Default)]
pub struct Alu {
    pub input1: Option<PortRef>,
    pub input2: Option<PortRef>,
    pub alu_ctl: Option<PortRef>,
}

impl Port for Alu {
    fn read(&self) -> u32 {
        let u1 = self.input1.as_ref().unwrap().read();
        let u2 = self.input2.as_ref().unwrap().read();
        let i1 = u2i(u1);
        let i2 = u2i(u2);
        let alu_ctl = self.alu_ctl.as_ref().unwrap().read();
        (match alu_ctl & 0b1 {
            0 => match (alu_ctl >> 1) & 0b111 {
                0 => match (alu_ctl >> 4) & 0b1 {
                    0 => i1 + i2,
                    1 => i1 - i2,
                    _ => panic!("Invalid ALU control signal"),
                },
                1 => i1 << (u2 & 0b11111),
                2 => (i1 < i2).into(),
                3 => (u1 < u2).into(),
                4 => i1 ^ i2,
                5 => match (alu_ctl >> 4) & 0b1 {
                    0 => u2i(u1 >> (u2 & 0b11111)),
                    1 => i1 >> (u2 & 0b11111),
                    _ => panic!("Invalid ALU control signal"),
                },
                6 => i1 | i2,
                7 => i1 & i2,
                _ => panic!("Invalid ALU control signal"),
            },
            1 => match (alu_ctl >> 1) & 0b111 {
                0 => i1 + i2,
                1 => i1 << (u2 & 0b11111),
                2 => (i1 < i2).into(),
                3 => (u1 < u2).into(),
                4 => i1 ^ i2,
                5 => match (alu_ctl >> 4) & 0b1 {
                    0 => u2i(u1 >> (i2 & 0b11111)),
                    1 => i1 >> (i2 & 0b11111),
                    _ => panic!("Invalid ALU control signal"),
                },
                6 => i1 | i2,
                7 => i1 & i2,
                _ => panic!("Invalid ALU control signal"),
            },
            _ => panic!("Invalid ALU control signal"),
        }) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::*;
    use crate::component::Builder;
    #[test]
    fn test_alu() {
        let mut alub = AluBuilder::default();
        let mut consts = ConstsBuilder::default();
        consts.push(1);
        consts.push(2);
        consts.push(0b1);
        alub.connect(consts.alloc(0), 1);
        alub.connect(consts.alloc(1), 2);
        alub.connect(consts.alloc(2), 0);
        assert_eq!(alub.alloc(0).read(), 3);
    }
}
