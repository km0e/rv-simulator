fn main() -> std::io::Result<()> {
    let pg = rv_simulator::init().expect("Failed to initialize simulator");
    let rv = rv_simulator::Rv32iBuilder::new(pg).slf_build();
    let mut backend = rv_simulator::tui::init()?;
    rv_simulator::tui::App::new(rv).run(&mut backend)?;
    rv_simulator::tui::restore()?;
    Ok(())
}
