use crate::circuit::Circuit;
use crate::component::{build::*, Control, ControlRef};

use self::control::ControlBuilder;
use self::decode::DecodeBuilder;
use self::imm::ImmBuilder;
use self::regs::RegsBuilder;
mod control;
mod decode;
mod imm;
mod regs;
use control::Alloc as ControlAlloc;
use decode::Alloc as DecodeAlloc;
use regs::Alloc as RegsAlloc;
use regs::Connect as RegsConnect;
pub enum Alloc {
    Rs1 = 0,
    Rs2 = 1,
    Rd = 2,
    Opcode = 3,
    Imm = 4,
    BranchType = 5,
    AluOp = 6,
    ImmSel = 7,
    PcSel = 8,
    BranchSel = 9,
    Jal_ = 10,
    //
    MemWrite = 12,
    WbSel = 13,
    RegWrite = 14,
    R1Data = 15,
    R2Data = 16,
}
impl From<Alloc> for usize {
    fn from(alloc: Alloc) -> usize {
        match alloc {
            Alloc::Rs1 => 0,
            Alloc::Rs2 => 1,
            Alloc::Rd => 2,
            Alloc::Opcode => 3,
            Alloc::Imm => 4,
            Alloc::BranchType => 5,
            Alloc::AluOp => 6,
            Alloc::ImmSel => 7,
            Alloc::PcSel => 8,
            Alloc::BranchSel => 9,
            Alloc::Jal_ => 10,
            Alloc::MemWrite => 12,
            Alloc::WbSel => 13,
            Alloc::RegWrite => 14,
            Alloc::R1Data => 15,
            Alloc::R2Data => 16,
        }
    }
}
pub enum Connect {
    Inst = 0,
    Rd = 1,
    RdData = 2,
    RegWrite = 3,
}
impl From<Connect> for usize {
    fn from(alloc: Connect) -> usize {
        match alloc {
            Connect::Inst => 0,
            Connect::Rd => 1,
            Connect::RdData => 2,
            Connect::RegWrite => 3,
        }
    }
}
pub struct IdBuilder {
    pub control: ControlBuilder,
    pub decode: DecodeBuilder,
    pub imm: ImmBuilder,
    pub regs: RegsBuilder,
}
impl IdBuilder {
    pub fn new() -> Self {
        // add id stage
        // set up decode
        let mut id_decode = DecodeBuilder::new();
        // set up imm
        let mut id_imm = ImmBuilder::default();
        id_imm.connect(id_decode.alloc(3), 0);
        // set up regs
        let mut id_regs = RegsBuilder::default();
        id_regs.connect(id_decode.alloc(0), 0);
        id_regs.connect(id_decode.alloc(1), 1);
        // set up control
        let mut id_control = ControlBuilder::new();
        id_control.connect(id_decode.alloc(3), 0);
        IdBuilder {
            control: id_control,
            decode: id_decode,
            imm: id_imm,
            regs: id_regs,
        }
    }
}
impl Builder for IdBuilder {
    fn alloc(&mut self, id: usize) -> ComponentRef {
        match id {
            0 => self.decode.alloc(DecodeAlloc::Rs1.into()),
            1 => self.decode.alloc(DecodeAlloc::Rs2.into()),
            2 => self.decode.alloc(DecodeAlloc::Rd.into()),
            3 => self.decode.alloc(DecodeAlloc::Opcode.into()),
            4 => self.imm.alloc(0),
            5 => self.control.alloc(ControlAlloc::BranchType.into()),
            6 => self.control.alloc(ControlAlloc::AluOp.into()),
            7 => self.control.alloc(ControlAlloc::ImmSel.into()),
            8 => self.control.alloc(ControlAlloc::PcSel.into()),
            9 => self.control.alloc(ControlAlloc::BranchSel.into()),
            10 => self.control.alloc(ControlAlloc::Jal_.into()),
            12 => self.control.alloc(ControlAlloc::MemWrite.into()),
            13 => self.control.alloc(ControlAlloc::WbSel.into()),
            14 => self.control.alloc(ControlAlloc::RegWrite.into()),
            15 => self.regs.alloc(RegsAlloc::R1Data.into()),
            16 => self.regs.alloc(RegsAlloc::R2Data.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn connect(&mut self, pin: ComponentRef, id: usize) {
        match id {
            0 => {
                self.decode.connect(pin.clone(), 0);
                self.imm.connect(pin.clone(), 1);
            }
            1 => self.regs.connect(pin.clone(), RegsConnect::Rd.into()),
            2 => self.regs.connect(pin.clone(), RegsConnect::RdData.into()),
            3 => self.regs.connect(pin.clone(), RegsConnect::Write.into()),
            _ => panic!("Invalid id"),
        }
    }
    fn build(self) -> Option<ControlRef> {
        self.regs.build()
    }
}
pub struct If {
    pub circuit: Circuit,
}
impl Control for If {
    fn rasing_edge(&mut self) {
        self.circuit.rasing_edge();
    }
    fn falling_edge(&mut self) {
        self.circuit.falling_edge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::build::*;
    #[test]
    fn test_generate_id1() {
        // instruction
        // sw x8 428(x2)
        let instruction = 0x1a812623;
        let mut idb = IdBuilder::new();
        let mut constb = ConstsBuilder::default();
        constb.push(instruction);
        constb.push(2);
        constb.push(1);
        constb.push(1);
        let rs1 = idb.alloc(Alloc::Rs1.into());
        let rs2 = idb.alloc(Alloc::Rs2.into());
        let rd = idb.alloc(Alloc::Rd.into());
        let opcode = idb.alloc(Alloc::Opcode.into());
        let imm = idb.alloc(Alloc::Imm.into());
        let alu_op = idb.alloc(Alloc::AluOp.into());
        let imm_sel = idb.alloc(Alloc::ImmSel.into());
        let pc_sel = idb.alloc(Alloc::PcSel.into());
        let branch_sel = idb.alloc(Alloc::BranchSel.into());
        let mem_write = idb.alloc(Alloc::MemWrite.into());
        let jal_ = idb.alloc(Alloc::Jal_.into());
        let wb_sel = idb.alloc(Alloc::WbSel.into());
        let reg_write = idb.alloc(Alloc::RegWrite.into());
        let r1_data = idb.alloc(Alloc::R1Data.into());
        let r2_data = idb.alloc(Alloc::R2Data.into());
        idb.connect(constb.alloc(0), Connect::Inst.into());
        idb.connect(constb.alloc(1), Connect::Rd.into());
        idb.connect(constb.alloc(2), Connect::RdData.into());
        idb.connect(constb.alloc(3), Connect::RegWrite.into());
        constb.build();
        let control = idb.build().unwrap();
        control.rasing_edge();
        control.falling_edge();
        assert_eq!(rs1.read(), 2);
        assert_eq!(rs2.read(), 8);
        assert_eq!(rd.read(), 12);
        // assert_eq!(opcode.read(), 0x46);
        assert_eq!(opcode.read(), instruction);
        assert_eq!(imm.read(), 428);
        assert_eq!(alu_op.read(), 0);
        assert_eq!(imm_sel.read(), 1);
        assert_eq!(pc_sel.read(), 0);
        assert_eq!(branch_sel.read(), 0);
        assert_eq!(mem_write.read(), 1);
        assert_eq!(jal_.read(), 0);
        // assert_eq!(wb_sel.read(), 0);
        assert_eq!(reg_write.read(), 0);
        assert_eq!(r1_data.read(), 1);
        assert_eq!(r2_data.read(), 0);
    }
    #[test]
    fn test_generate_id2() {
        // instruction
        // jal x0 40
        let instruction = 0x280006f;
        let mut idb = IdBuilder::new();
        let mut constb = ConstsBuilder::default();
        constb.push(instruction);
        constb.push(0);
        constb.push(0);
        constb.push(1);
        let rs1 = idb.alloc(Alloc::Rs1.into());
        let rs2 = idb.alloc(Alloc::Rs2.into());
        let rd = idb.alloc(Alloc::Rd.into());
        let opcode = idb.alloc(Alloc::Opcode.into());
        let imm = idb.alloc(Alloc::Imm.into());
        let alu_op = idb.alloc(Alloc::AluOp.into());
        let imm_sel = idb.alloc(Alloc::ImmSel.into());
        let pc_sel = idb.alloc(Alloc::PcSel.into());
        let branch_sel = idb.alloc(Alloc::BranchSel.into());
        let mem_write = idb.alloc(Alloc::MemWrite.into());
        let jal_ = idb.alloc(Alloc::Jal_.into());
        let wb_sel = idb.alloc(Alloc::WbSel.into());
        let reg_write = idb.alloc(Alloc::RegWrite.into());
        let r1_data = idb.alloc(Alloc::R1Data.into());
        let r2_data = idb.alloc(Alloc::R2Data.into());
        idb.connect(constb.alloc(0), Connect::Inst.into());
        idb.connect(constb.alloc(1), Connect::Rd.into());
        idb.connect(constb.alloc(2), Connect::RdData.into());
        idb.connect(constb.alloc(3), Connect::RegWrite.into());
        constb.build();
        let control = idb.build().unwrap();
        control.rasing_edge();
        control.falling_edge();
        assert_eq!(rs1.read(), 0);
        assert_eq!(rs2.read(), 8);
        assert_eq!(rd.read(), 0);
        // assert_eq!(opcode.read(), 0x6f);
        assert_eq!(opcode.read(), instruction);
        assert_eq!(imm.read(), 40);
        assert_eq!(alu_op.read(), 0);
        assert_eq!(imm_sel.read(), 1);
        assert_eq!(pc_sel.read(), 1);
        assert_eq!(branch_sel.read(), 0);
        assert_eq!(mem_write.read(), 0);
        assert_eq!(jal_.read(), 1);
        // assert_eq!(wb_sel.read(), 1);
        assert_eq!(reg_write.read(), 1);
        assert_eq!(r1_data.read(), 0);
        assert_eq!(r2_data.read(), 0);
    }
    #[test]
    fn test_generate_id3() {
        // instruction
        //  addi x8 x2 432
        let instruction = 0x1b010413;
        let mut idb = IdBuilder::new();
        let mut constb = ConstsBuilder::default();
        constb.push(instruction);
        constb.push(2);
        constb.push(1);
        constb.push(1);
        let rs1 = idb.alloc(Alloc::Rs1.into());
        let rs1 = idb.alloc(Alloc::Rs1.into());
        let rs2 = idb.alloc(Alloc::Rs2.into());
        let rd = idb.alloc(Alloc::Rd.into());
        let opcode = idb.alloc(Alloc::Opcode.into());
        let imm = idb.alloc(Alloc::Imm.into());
        let alu_op = idb.alloc(Alloc::AluOp.into());
        let imm_sel = idb.alloc(Alloc::ImmSel.into());
        let pc_sel = idb.alloc(Alloc::PcSel.into());
        let branch_sel = idb.alloc(Alloc::BranchSel.into());
        let mem_write = idb.alloc(Alloc::MemWrite.into());
        let jal_ = idb.alloc(Alloc::Jal_.into());
        let wb_sel = idb.alloc(Alloc::WbSel.into());
        let reg_write = idb.alloc(Alloc::RegWrite.into());
        let r1_data = idb.alloc(Alloc::R1Data.into());
        let r2_data = idb.alloc(Alloc::R2Data.into());
        idb.connect(constb.alloc(0), Connect::Inst.into());
        idb.connect(constb.alloc(1), Connect::Rd.into());
        idb.connect(constb.alloc(2), Connect::RdData.into());
        idb.connect(constb.alloc(3), Connect::RegWrite.into());
        constb.build();
        let control = idb.build().unwrap();
        control.rasing_edge();
        control.falling_edge();
        assert_eq!(rs1.read(), 2);
        assert_eq!(rs2.read(), 16);
        assert_eq!(rd.read(), 8);
        // assert_eq!(opcode.read(), 0x13);
        assert_eq!(opcode.read(), instruction);
        assert_eq!(imm.read(), 432);
        assert_eq!(alu_op.read(), 1);
        assert_eq!(imm_sel.read(), 1);
        assert_eq!(pc_sel.read(), 0);
        assert_eq!(branch_sel.read(), 0);
        assert_eq!(mem_write.read(), 0);
        assert_eq!(jal_.read(), 0);
        // assert_eq!(wb_sel.read(), 0);
        assert_eq!(reg_write.read(), 1);
        assert_eq!(r1_data.read(), 1);
        assert_eq!(r2_data.read(), 0);
    }
}
