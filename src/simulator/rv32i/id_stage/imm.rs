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
    fn alloc(&mut self, id: Alloc) -> PortRef {
        PortRef::from(self.inner.clone())
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Opcode => self.inner.borrow_mut().opcode = Some(pin),
            Connect::Inst => self.inner.borrow_mut().inst = Some(pin),
            _ => panic!("Invalid id"),
        }
    }
}

#[derive(Default)]
pub struct Imm {
    pub opcode: Option<PortRef>,
    pub inst: Option<PortRef>,
}

impl Port for Imm {
    fn read(&self) -> u32 {
        let input = match self.opcode {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let inst = match self.inst {
            Some(ref input) => input.read(),
            None => {
                unimplemented!()
            }
        };
        let inst = inst as i32;
        (match input & 0b1111111 {
            0b001_0011 | 0b000_0011 | 0b110_0111 => inst >> 20,
            0b010_0011 => ((inst >> 7) & 0b11111) | (((inst as i32) >> 25) << 5),
            0b110_0011 => {
                ((inst >> 7) & 0b11110)
                    | (((inst >> 25) & 0b111111) << 5)
                    | (((inst >> 7) & 0b1) << 11)
                    | (((inst >> 31) & 0b1) << 12)
            }
            0b110_1111 => {
                ((inst >> 20) & 0b11111111110)
                    | ((inst >> 9) & 0b100000000000)
                    | ((inst) & 0b11111111000000000000)
                    | ((inst >> 10) & 0b100000000000000000000)
            }
            _ => inst >> 12,
        }) as u32
    }
}
pub mod build {
    pub use super::Alloc as ImmAlloc;
    pub use super::Connect as ImmConnect;
    pub use super::ImmBuilder;
}
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
