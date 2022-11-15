pub fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    return true
}
pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut number = 2;
    while count != n {
        number += 1;
        if is_prime(number) {
            count += 1
        }
    }
    number
}