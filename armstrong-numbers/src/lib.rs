pub fn is_armstrong_number(num: u32) -> bool {
    let digits :u32 = (num as f32).log10() as u32 + 1;
    let mut temp = num;
    let mut sum :u32 = 0;
    while temp > 0 {
        sum += (temp % 10).pow(digits);
        temp /= 10;
    }
    sum == num
}
