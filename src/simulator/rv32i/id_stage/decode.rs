use crate::common::abi::*;
use crate::common::build::*;

pub enum Alloc {
    Rs1 = 0,
    Rs2 = 1,
    Rd = 2,
    Opcode = 3,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Rs1 => 0,
            Alloc::Rs2 => 1,
            Alloc::Rd => 2,
            Alloc::Opcode => 3,
        }
    }
}

pub enum Connect {
    Inst = 0,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Inst => 0,
        }
    }
}
pub struct DecodeBuilder {
    rs1: BitBuilder,
    rs2: BitBuilder,
    rd: BitBuilder,
    opcode: BitBuilder,
}
impl DecodeBuilder {
    pub fn new() -> Self {
        Self {
            rs1: BitBuilder::new((15, 19)),
            rs2: BitBuilder::new((20, 24)),
            rd: BitBuilder::new((7, 11)),
            opcode: BitBuilder::new((0, 31)),
        }
    }
}
impl Default for DecodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl PortBuilder for DecodeBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::Rs1 => self.rs1.alloc(BitAlloc::Out),
            Alloc::Rs2 => self.rs2.alloc(BitAlloc::Out),
            Alloc::Rd => self.rd.alloc(BitAlloc::Out),
            Alloc::Opcode => self.opcode.alloc(BitAlloc::Out),
        }
    }
    fn connect(&mut self, pin: PortRef, _id: Connect) {
        self.rs1.connect(pin.clone(), BitConnect::In);
        self.rs2.connect(pin.clone(), BitConnect::In);
        self.rd.connect(pin.clone(), BitConnect::In);
        self.opcode.connect(pin.clone(), BitConnect::In);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode1() {
        let mut tb = DecodeBuilder::new();
        let mut constant = ConstsBuilder::default();
        // 0000000 | 00001 | 00010 | 000 | 00011 | 0110011
        //    0    |   1   |   2   |  0  |   3   | 0110011
        constant.push(0b00000000000100010000000110110011);
        let rs1 = tb.alloc(Alloc::Rs1);
        let rs2 = tb.alloc(Alloc::Rs2);
        let rd = tb.alloc(Alloc::Rd);
        let opcode = tb.alloc(Alloc::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::Inst);
        assert_eq!(rs1.read(), 2);
        assert_eq!(rs2.read(), 1);
        assert_eq!(rd.read(), 3);
        // assert_eq!(opcode.read(), 0b0110011);//todo pack opcode
        assert_eq!(opcode.read(), 0b00000000000100010000000110110011); //todo pack opcode
    }
    #[test]
    fn test_decode2() {
        let instruction = 0x280006f;
        let mut tb = DecodeBuilder::new();
        let mut constant = ConstsBuilder::default();
        // 0000001 | 01000 | 00000 | 000 | 00000 | 1101111
        //    1    |   8   |   0   |  0  |   0   | 1101111
        constant.push(instruction);
        let rs1 = tb.alloc(Alloc::Rs1);
        let rs2 = tb.alloc(Alloc::Rs2);
        let rd = tb.alloc(Alloc::Rd);
        let opcode = tb.alloc(Alloc::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::Inst);
        assert_eq!(rs1.read(), 0);
        assert_eq!(rs2.read(), 8);
        assert_eq!(rd.read(), 0);
        // assert_eq!(opcode.read(), 0b1101111);//todo pack opcode
        assert_eq!(opcode.read(), instruction); //todo pack opcode
    }
}
