use super::Component;

#[derive(Default)]
pub struct Bomb {}

impl Component for Bomb {
    fn read(&self) -> u32 {
        unimplemented!()
    }
}
