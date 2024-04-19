use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub compiler: String,
    pub objdump: String,
    pub file: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            compiler: String::from("riscv32-unknown-elf-gcc"),
            objdump: String::from("riscv32-unknown-elf-objdump"),
            file: String::from("main.c"),
        }
    }
}

pub fn init() -> Config {
    xcfg::load::<Config>("config.").ok().unwrap_or_default()
}
