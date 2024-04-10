use crate::component::build::MuxBuilder;
mod alu;
pub struct ExBuilder {
    pub forward1: MuxBuilder,
    pub forward2: MuxBuilder,
    pub pc_sel: MuxBuilder,
    pub imm_sel: MuxBuilder,
}
