pub trait Status {
    fn input(&self) -> Vec<(String, u32)>;
    fn output(&self) -> Vec<(String, u32)>;
    fn inout(&self) -> Vec<(String, u32, u32)>;
}
