fn main() -> std::io::Result<()> {
    let instruction_memory: Vec<u8> = vec![
        0xe5010113, 0x1a812623, 0x1b010413, 0xfe042623, 0x0280006f, 0xfec42783, 0x00279793,
        0xff040713, 0x00f707b3, 0xfec42703, 0xe6e7a623, 0xfec42783, 0x00178793, 0xfef42623,
        0xfec42703, 0x06300793, 0xfce7dae3, 0x00100793, 0xfef42623, 0x0400006f, 0xfec42783,
    ]
    .into_iter()
    .flat_map(|x: u32| x.to_ne_bytes())
    .collect();
    let asm_mem = vec![
        "e5010113 addi x2 x2 -432".to_string(),
        "1a812623 sw x8 428 x2".to_string(),
        "1b010413 addi x8 x2 432".to_string(),
        "fe042623 sw x0 -20 x8".to_string(),
        "0280006f jal x0 40".to_string(),
        "fec42783 lw x15 -20 x8".to_string(),
        "00279793 slli x15 x15 2".to_string(),
        "ff040713 addi x14 x8 -16".to_string(),
        "00f707b3 add x15 x14 x15".to_string(),
        "fec42703 lw x14 -20 x8".to_string(),
        "e6e7a623 sw x14 -404 x15".to_string(),
        "fec42783 lw x15 -20 x8".to_string(),
        "00178793 addi x15 x15 1".to_string(),
        "fef42623 sw x15 -20 x8".to_string(),
        "fec42703 lw x14 -20 x8".to_string(),
        "06300793 addi x15 x0 99".to_string(),
        "fce7dae3 bge x15 x14 -44".to_string(),
        "00100793 addi x15 x0 1".to_string(),
        "fef42623 sw x15 -20 x8".to_string(),
        "0400006f jal x0 64".to_string(),
        "fec42783 lw x15 -20 x8".to_string(),
    ];
    let rv = rv_simulator::Rv32iBuilder::new(instruction_memory, asm_mem).slf_build();
    let mut backend = rv_simulator::tui::init()?;
    rv_simulator::tui::App::new(rv).run(&mut backend)?;
    rv_simulator::tui::restore()?;
    Ok(())
}
