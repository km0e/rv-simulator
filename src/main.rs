fn main() -> std::io::Result<()> {
    let instruction_memory: Vec<u32> = vec![
        0xe5010113, // addi x2 x2 -432
        0x1a812623, // sw x8 428 x2
        0x1b010413, // addi x8 x2 432
        0xfe042623, // sw x0 -20 x8
        0x0280006f, // jal x0 40
        0xfec42783, // lw x15 -20 x8
    ];
    let asm_mem = vec![
        "addi x2 x2 -432".to_string(),
        "sw x8 428 x2".to_string(),
        "addi x8 x2 432".to_string(),
        "sw x0 -20 x8".to_string(),
        "jal x0 40".to_string(),
        "lw x15 -20 x8".to_string(),
    ];
    let rv = rv_simulator::Rv32iBuilder::new(instruction_memory, asm_mem).slf_build();
    let mut backend = rv_simulator::tui::init()?;
    rv_simulator::tui::App::new(rv).run(&mut backend)?;
    rv_simulator::tui::restore()
}
