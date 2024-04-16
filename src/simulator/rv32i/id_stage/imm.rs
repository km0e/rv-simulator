use crate::common::abi::*;
pub enum Alloc {
    Out,
}
pub enum Connect {
    Opcode,
    Inst,
}
#[derive(Default)]
pub struct ImmBuilder {
    inner: PortShared<Imm>,
}
impl PortBuilder for ImmBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, _id: Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Opcode => self.inner.borrow_mut().opcode = Some(pin),
            Connect::Inst => self.inner.borrow_mut().inst = Some(pin),
        }
    }
}

#[derive(Default, Debug)]
pub struct Imm {
    pub opcode: Option<PortRef>,
    pub inst: Option<PortRef>,
}
impl Port for Imm {
    fn read(&self) -> u32 {
        let inst = match self.opcode {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let input = match self.inst {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let input = input as i32;
        let output = match inst & 0b1111111 {
            0b001_0011 | 0b000_0011 | 0b110_0111 => input >> 20,
            0b010_0011 => ((input >> 7) & 0b11111) | (((input as i32) >> 25) << 5),
            //|imm[12]|imm[10:5]|rs2|rs1|funct3|imm[4:1]|imm[11]|opcode|
            0b110_0011 => {
                ((input >> 7) & 0b11110)
                    | (((input >> 25) & 0b111111) << 5)
                    | (((input >> 7) & 0b1) << 11)
                    | ((input >> 31) << 12)
            }
            0b110_1111 => {
                ((input >> 20) & 0b11111111110)
                    | ((input >> 9) & 0b100000000000)
                    | ((input) & 0b11111111000000000000)
                    | ((input >> 10) & 0b100000000000000000000)
            }
            _ => input >> 12,
        };
        output as u32
    }
}
pub mod build {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    #[test]
    fn test_imm() {
        let mut tb = ImmBuilder::default();
        let imm = tb.alloc(Alloc::Out);
        let mut constant = ConstsBuilder::default();
        // I-type: addi x3, x1, 2
        // 0000000 | 00010 | 00001 | 000 | 00011 | 0010011
        constant.push(0b001_0011);
        constant.push(0b00000000001000001000000100000011);
        tb.connect(constant.alloc(ConstsAlloc::Out(0)), Connect::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(1)), Connect::Inst);
        assert_eq!(imm.read(), 2);
        // S-type: sw x1, -4(x2)
        // 1111111 | 00010 | 00001 | 010 | 11100 | 0100011
        constant.push(0b010_0011);
        constant.push(0b11111110001000001010111000100011);
        tb.connect(constant.alloc(ConstsAlloc::Out(2)), Connect::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(3)), Connect::Inst);
        assert_eq!(imm.read() as i32, -4);
        // S-type: sw x8, 428(x2)
        // 0001101 | 01000 | 00010 | 010 | 01100 | 0100011
        constant.push(0b010_0011);
        constant.push(0b00011010100000010010011000100011);
        tb.connect(constant.alloc(ConstsAlloc::Out(4)), Connect::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(5)), Connect::Inst);
        assert_eq!(imm.read(), 428);
        // U-type: lui x1, 0x2346
        // 0011010 | 00110 | 00000 | 010 | 00001 | 1101111
        constant.push(0b110_1111);
        constant.push(0b00110100011000000010000011101111);
        tb.connect(constant.alloc(ConstsAlloc::Out(6)), Connect::Opcode);
        tb.connect(constant.alloc(ConstsAlloc::Out(7)), Connect::Inst);
        assert_eq!(imm.read(), 0x2346);
    }
}
