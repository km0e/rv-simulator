use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub compiler: Option<String>,
    #[arg(short, long)]
    pub objdump: Option<String>,
    #[arg(short, long)]
    pub file: Option<String>,
}

pub fn init() -> Args {
    Args::parse()
}
