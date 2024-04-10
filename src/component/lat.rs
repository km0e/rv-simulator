use super::Component;

#[derive(Default)]
pub struct Lat {
    pub data: u32,
}
impl Lat {
    pub fn new(value: u32) -> Self {
        Self { data: value }
    }
}
impl Component for Lat {
    fn read(&self) -> u32 {
        self.data
    }
}
