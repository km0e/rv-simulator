use crate::component::{Component, ComponentRef};

pub struct AluBuilder {}
pub struct Alu {
    pub input1: Option<ComponentRef>,
    pub input2: Option<ComponentRef>,
    pub alu_ctl: Option<ComponentRef>,
}
