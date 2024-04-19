use goblin::{elf, Object};
use std::process::Command;
mod args;
mod file;
#[derive(Debug, Default, Clone)]
pub struct Program {
    pub insts: Vec<u8>,
    pub start: usize,
    pub asm: String,
    pub entry: usize,
}
pub fn init() -> Result<Program, ()> {
    let args = args::init();
    let file = file::init();
    let compiler = args.compiler_path.unwrap_or(file.compiler);
    let objdump = args.objdump_path.unwrap_or(file.objdump);
    let file = args.file.unwrap_or(file.file);
    let mut pg = Program::default();
    let status = Command::new(compiler)
        .args([
            "-march=rv32i",
            "-mabi=ilp32",
            "-O0",
            "-x",
            "c",
            "-static",
            "-nostdlib",
            "-nostartfiles",
        ])
        .arg(file)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    if !status.success() {
        return Err(());
    }
    let dat = std::fs::read("a.out").unwrap();
    match Object::parse(&dat).unwrap() {
        Object::Elf(elf) => {
            pg.entry = elf.entry as usize;
            for sh in elf.section_headers {
                if sh.sh_type == elf::section_header::SHT_PROGBITS
                    && &elf.shdr_strtab[sh.sh_name] == ".text"
                {
                    pg.start = sh.sh_addr as usize;
                    pg.insts.extend_from_slice(
                        &dat[sh.sh_offset as usize..(sh.sh_offset + sh.sh_size) as usize],
                    );
                    break;
                }
            }
        }
        _ => return Err(()),
    }
    let status = Command::new(objdump)
        .args(["-d", "a.out", "-M", "numeric"])
        .output()
        .unwrap();
    if !status.status.success() {
        return Err(());
    }
    let stdout = String::from_utf8(status.stdout).unwrap();
    let pos = stdout.find("Disassembly of section .text:").unwrap();
    pg.asm = stdout[pos + 30..].to_string();
    Ok(pg)
}
