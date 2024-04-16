use crate::common::abi::*;
use crate::common::utils::u2i;
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
    inner: ControlShared<Alu>,
}
impl ControlBuilder for AluBuilder {
    fn build(self) -> ControlRef {
        self.inner.into_shared().into()
    }
}
impl PortBuilder for AluBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Self::Alloc) -> PortRef {
        match id {
            Self::Alloc::Res => self.inner.clone().into_shared().into(),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Self::Connect) {
        match id {
            Self::Connect::Ctrl => self.inner.borrow_mut().alu_ctl = Some(pin.clone()),
            Self::Connect::Op1 => self.inner.borrow_mut().input1 = Some(pin.clone()),
            Self::Connect::Op2 => self.inner.borrow_mut().input2 = Some(pin.clone()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Alu {
    pub input1: Option<PortRef>,
    pub input2: Option<PortRef>,
    pub alu_ctl: Option<PortRef>,
}
impl Control for Alu {
    fn output(&self) -> Vec<(String, u32)> {
        vec![("res".to_string(), self.read())]
    }
}
impl Port for Alu {
    fn read(&self) -> u32 {
        let u1 = self.input1.as_ref().unwrap().read();
        let u2 = self.input2.as_ref().unwrap().read();
        let i1 = u2i(u1);
        let i2 = u2i(u2);
        let alu_ctl = self.alu_ctl.as_ref().unwrap().read();
        (match alu_ctl & 0b1 {
            0 => 0,
            1 => match (alu_ctl >> 1) & 0b111 {
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
    use crate::common::build::*;
    #[test]
    fn test_alu() {
        let mut alub = AluBuilder::default();
        let mut consts = ConstsBuilder::default();
        consts.push(1);
        consts.push(2);
        consts.push(0b1);
        alub.connect(consts.alloc(ConstsAlloc::Out(0)), Connect::Op1);
        alub.connect(consts.alloc(ConstsAlloc::Out(1)), Connect::Op2);
        alub.connect(consts.alloc(ConstsAlloc::Out(2)), Connect::Ctrl);
        assert_eq!(alub.alloc(Alloc::Res).read(), 3);
    }
}
