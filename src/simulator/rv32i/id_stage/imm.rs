use super::{Builder, Port, PortRef, PortShared};
#[derive(Default)]
pub struct ImmBuilder {
    inner: PortShared<Imm>,
}
impl Builder for ImmBuilder {
    fn alloc(&mut self, id: usize) -> PortRef {
        assert_eq!(id, 0);
        PortRef::from(self.inner.clone())
    }
    fn build(self) -> Option<crate::component::ControlRef> {
        None
    }
    fn connect(&mut self, pin: PortRef, id: usize) {
        match id {
            0 => self.inner.borrow_mut().opcode = Some(pin),
            1 => self.inner.borrow_mut().inst = Some(pin),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::*;
    #[test]
    fn test_imm() {
        let mut tb = ImmBuilder::default();
        let imm = tb.alloc(0);
        let mut constant = ConstsBuilder::default();
        // I-type: addi x3, x1, 2
        // 0000000 | 00010 | 00001 | 000 | 00011 | 0010011
        constant.push(0b001_0011);
        constant.push(0b00000000001000001000000100000011);
        tb.connect(constant.alloc(0), 0);
        tb.connect(constant.alloc(1), 1);
        assert_eq!(imm.read(), 2);
        // S-type: sw x1, -4(x2)
        // 1111111 | 00010 | 00001 | 010 | 11100 | 0100011
        constant.push(0b010_0011);
        constant.push(0b11111110001000001010111000100011);
        tb.connect(constant.alloc(2), 0);
        tb.connect(constant.alloc(3), 1);
        assert_eq!(imm.read() as i32, -4);
        // S-type: sw x8, 428(x2)
        // 0001101 | 01000 | 00010 | 010 | 01100 | 0100011
        constant.push(0b010_0011);
        constant.push(0b00011010100000010010011000100011);
        tb.connect(constant.alloc(4), 0);
        tb.connect(constant.alloc(5), 1);
        assert_eq!(imm.read(), 428);
        // U-type: lui x1, 0x2346
        // 0011010 | 00110 | 00000 | 010 | 00001 | 1101111
        constant.push(0b110_1111);
        constant.push(0b00110100011000000010000011101111);
        tb.connect(constant.alloc(6), 0);
        tb.connect(constant.alloc(7), 1);
        assert_eq!(imm.read(), 0x2346);
    }
}
