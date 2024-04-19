pub fn u2i(u: u32) -> i32 {
    unsafe { std::mem::transmute::<u32, i32>(u) }
}
// pub fn i2u(i: i32) -> u32 {
//     unsafe { std::mem::transmute::<i32, u32>(i) }
// }
