mod ex_mem;
mod id_ex;
mod if_id;
mod mem_wb;

pub mod build {
    pub use super::ex_mem::build::*;
    pub use super::id_ex::build::*;
    pub use super::if_id::build::*;
    pub use super::mem_wb::build::*;
}
