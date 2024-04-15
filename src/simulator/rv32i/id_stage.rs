use crate::common::abi::*;

use control::Alloc as CtrlAlloc;
use control::Connect as CtrlConnect;
use control::CtrlSigBuilder;
use decode::Alloc as DecodeAlloc;
use decode::Connect as DecodeConnect;
use decode::DecodeBuilder;
use imm::Alloc as ImmAlloc;
use imm::Connect as ImmConnect;
use imm::ImmBuilder;
use xregs::Alloc as XregsAlloc;
use xregs::Connect as XregsConnect;
use xregs::XregsBuilder;
mod control;
mod decode;
mod imm;
mod xregs;
pub enum Alloc {
    Rs1 = 0,
    Rs2 = 1,
    Rd = 2,
    Opcode = 3,
    Imm = 4,
    BranchType = 5,
    AluCtrl = 6,
    ImmSel = 7,
    PcSel = 8,
    BranchEn = 9,
    Jal_ = 10,
    //
    MemWrite = 12,
    WbSel = 13,
    RegWrite = 14,
    Rs1Data = 15,
    Rs2Data = 16,
    Load = 17,
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
pub struct IdStageBuilder {
    pub control: CtrlSigBuilder,
    pub decode: DecodeBuilder,
    pub imm: ImmBuilder,
    pub xregs: XregsBuilder,
}
impl IdStageBuilder {
    pub fn new(esp: u32) -> Self {
        // add id stage
        // set up decode
        let mut id_decode = DecodeBuilder::new();
        // set up imm
        let mut id_imm = ImmBuilder::default();
        id_imm.connect(id_decode.alloc(DecodeAlloc::Opcode), ImmConnect::Opcode);
        // set up regs
        let mut id_regs = XregsBuilder::new(esp);
        id_regs.connect(id_decode.alloc(DecodeAlloc::Rs1), XregsConnect::Rs1);
        id_regs.connect(id_decode.alloc(DecodeAlloc::Rs2), XregsConnect::Rs2);
        // set up control
        let mut id_control = CtrlSigBuilder::new();
        id_control.connect(id_decode.alloc(DecodeAlloc::Opcode), CtrlConnect::Opcode);
        IdStageBuilder {
            control: id_control,
            decode: id_decode,
            imm: id_imm,
            xregs: id_regs,
        }
    }
}
impl PortBuilder for IdStageBuilder {
    type Alloc = Alloc;
    type Connect = Connect;
    fn alloc(&mut self, id: Alloc) -> PortRef {
        match id {
            Alloc::Rs1 => self.decode.alloc(DecodeAlloc::Rs1),
            Alloc::Rs2 => self.decode.alloc(DecodeAlloc::Rs2),
            Alloc::Rd => self.decode.alloc(DecodeAlloc::Rd),
            Alloc::Opcode => self.decode.alloc(DecodeAlloc::Opcode),
            Alloc::Imm => self.imm.alloc(ImmAlloc::Out),
            Alloc::BranchType => self.control.alloc(CtrlAlloc::BranchType),
            Alloc::AluCtrl => self.control.alloc(CtrlAlloc::AluCtrl),
            Alloc::ImmSel => self.control.alloc(CtrlAlloc::ImmSel),
            Alloc::PcSel => self.control.alloc(CtrlAlloc::PcSel),
            Alloc::BranchEn => self.control.alloc(CtrlAlloc::BranchEn),
            Alloc::Jal_ => self.control.alloc(CtrlAlloc::Jal_),
            Alloc::MemWrite => self.control.alloc(CtrlAlloc::MemWrite),
            Alloc::WbSel => self.control.alloc(CtrlAlloc::WbSel),
            Alloc::RegWrite => self.control.alloc(CtrlAlloc::RegWrite),
            Alloc::Rs1Data => self.xregs.alloc(XregsAlloc::R1Data),
            Alloc::Rs2Data => self.xregs.alloc(XregsAlloc::R2Data),
            Alloc::Load => self.control.alloc(CtrlAlloc::Load),
        }
    }
    fn connect(&mut self, pin: PortRef, id: Connect) {
        match id {
            Connect::Inst => {
                self.decode.connect(pin.clone(), DecodeConnect::Inst);
                self.imm.connect(pin.clone(), ImmConnect::Inst);
            }
            Connect::Rd => self.xregs.connect(pin.clone(), XregsConnect::Rd),
            Connect::RdData => self.xregs.connect(pin.clone(), XregsConnect::RdData),
            Connect::RegWrite => self.xregs.connect(pin.clone(), XregsConnect::Write),
        }
    }
}
impl ControlBuilder for IdStageBuilder {
    fn build(self) -> ControlRef {
        self.xregs.build()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::build::*;

    struct TestConnect {
        pub inst: u32,
        pub rd: u32,
        pub rd_data: u32,
        pub reg_write: u32,
    }
    struct TestAlloc {
        pub rs1: u32,
        pub rs2: u32,
        pub rd: u32,
        pub opcode: u32,
        pub imm: u32,
        pub branch_type: u32,
        pub alu_ctrl: u32,
        pub imm_sel: u32,
        pub pc_sel: u32,
        pub branch_en: u32,
        pub jal_: u32,
        pub mem_write: u32,
        pub wb_sel: u32,
        pub reg_write: u32,
        pub rs1_data: u32,
        pub rs2_data: u32,
        pub load: u32,
    }
    fn run_test(test_alloc: TestAlloc, test_connect: TestConnect) {
        let mut idb = IdStageBuilder::new(0x7ffffff0);
        let mut constb = ConstsBuilder::default();
        constb.push(test_connect.inst);
        constb.push(test_connect.rd);
        constb.push(test_connect.rd_data);
        constb.push(test_connect.reg_write);
        idb.connect(constb.alloc(ConstsAlloc::Out(0)), Connect::Inst);
        idb.connect(constb.alloc(ConstsAlloc::Out(1)), Connect::Rd);
        idb.connect(constb.alloc(ConstsAlloc::Out(2)), Connect::RdData);
        idb.connect(constb.alloc(ConstsAlloc::Out(3)), Connect::RegWrite);
        let rs1 = idb.alloc(Alloc::Rs1);
        let rs2 = idb.alloc(Alloc::Rs2);
        let rd = idb.alloc(Alloc::Rd);
        let opcode = idb.alloc(Alloc::Opcode);
        let imm = idb.alloc(Alloc::Imm);
        let branch_type = idb.alloc(Alloc::BranchType);
        let alu_ctrl = idb.alloc(Alloc::AluCtrl);
        let imm_sel = idb.alloc(Alloc::ImmSel);
        let pc_sel = idb.alloc(Alloc::PcSel);
        let branch_en = idb.alloc(Alloc::BranchEn);
        let jal_ = idb.alloc(Alloc::Jal_);
        let mem_write = idb.alloc(Alloc::MemWrite);
        let wb_sel = idb.alloc(Alloc::WbSel);
        let reg_write = idb.alloc(Alloc::RegWrite);
        let rs1_data = idb.alloc(Alloc::Rs1Data);
        let rs2_data = idb.alloc(Alloc::Rs2Data);
        let load = idb.alloc(Alloc::Load);
        idb.build();
        assert_eq!(rs1.read(), test_alloc.rs1);
        assert_eq!(rs2.read(), test_alloc.rs2);
        assert_eq!(rd.read(), test_alloc.rd);
        assert_eq!(opcode.read(), test_alloc.opcode);
        assert_eq!(imm.read(), test_alloc.imm);
        assert_eq!(branch_type.read(), test_alloc.branch_type);
        assert_eq!(alu_ctrl.read(), test_alloc.alu_ctrl);
        assert_eq!(imm_sel.read(), test_alloc.imm_sel);
        assert_eq!(pc_sel.read(), test_alloc.pc_sel);
        assert_eq!(branch_en.read(), test_alloc.branch_en);
        assert_eq!(jal_.read(), test_alloc.jal_);
        assert_eq!(mem_write.read(), test_alloc.mem_write);
        assert_eq!(wb_sel.read(), test_alloc.wb_sel);
        assert_eq!(reg_write.read(), test_alloc.reg_write);
        assert_eq!(rs1_data.read(), test_alloc.rs1_data);
        assert_eq!(rs2_data.read(), test_alloc.rs2_data);
        assert_eq!(load.read(), test_alloc.load);
    }
    #[test]
    fn test_generate_id0() {
        let test_alloc = TestAlloc {
            rs1: 2,
            rs2: 0x10,
            rd: 0x02,
            opcode: 0xe5010113,
            imm: 0xfffffe50,
            branch_type: 0,
            alu_ctrl: 1,
            imm_sel: 1,
            pc_sel: 0,
            branch_en: 0,
            jal_: 0,
            mem_write: 0,
            wb_sel: 1,
            reg_write: 1,
            rs1_data: 0x7ffffff0,
            rs2_data: 0,
            load: 0,
        };
        let test_connect = TestConnect {
            inst: 0xe5010113,
            rd: 0,
            rd_data: 0,
            reg_write: 0,
        };
        run_test(test_alloc, test_connect);
    }
    #[test]
    fn test_generate_id1() {
        let test_alloc = TestAlloc {
            rs1: 2,
            rs2: 8,
            rd: 0xc,
            opcode: 0x1a812623,
            imm: 0x1ac,
            branch_type: 2, //ripes is 0
            alu_ctrl: 1,
            imm_sel: 1,
            pc_sel: 0,
            branch_en: 0,
            jal_: 0,
            mem_write: 1,
            wb_sel: 1,
            reg_write: 0,
            rs1_data: 0x7ffffff0,
            rs2_data: 0,
            load: 0,
        };
        let test_connect = TestConnect {
            inst: 0x1a812623,
            rd: 0,
            rd_data: 0,
            reg_write: 0,
        };
        run_test(test_alloc, test_connect);
    }
}
