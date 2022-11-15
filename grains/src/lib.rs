pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        65..=u32::MAX => panic!("Square must be between 1 and 64"),
        _ => square_method(s),
    }
   
  
}
fn square_method(s:u32)->u64{
    u64::pow(2, s-1)
}

pub fn total() -> u64 {
   (1..=64).map(square_method).sum()
}
