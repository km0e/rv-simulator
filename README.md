# rv-simulator
this is a simple risc-v simulator. it can simulate rv32i instructions.

## plan
- [x] rv32i

## build
```shell
cargo build --release
```

## run
```shell
> ./rv-simulator -h
Usage: rv-simulator [OPTIONS]

Options:
  -c, --compiler <COMPILER>  
  -o, --objdump <OBJDUMP>    
  -f, --file <FILE>          
  -h, --help                 Print help
  -V, --version              Print version
```
config by cmd args or config file "config.toml"(yaml)
```toml
compiler = "riscv32-unknown-elf-gcc"
objdump = "riscv32-unknown-elf-objdump"
file = "main.c"
```
