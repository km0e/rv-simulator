use super::Port;

#[derive(Default)]
pub struct Bomb {}

impl Port for Bomb {
    fn read(&self) -> u32 {
        unimplemented!()
    }
}
