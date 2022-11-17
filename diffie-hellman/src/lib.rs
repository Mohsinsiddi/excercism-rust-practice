static mut STARTER: u64 = 2;

pub fn private_key(p: u64) -> u64 {
    prime_gen(p)
}

fn is_prime(n: u64) -> bool {
    n == 2 || n % 2 > 0 && (3..=(n as f64).sqrt() as u64).step_by(2).all(|i| n % i > 0)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    power_mod(g,a,p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
   power_mod(b_pub, a, p)
}

fn prime_gen(end: u64) -> u64 {
    let mut i;
    unsafe {
        i = STARTER;
    }
    loop {
        
        if i == end {
            i = 2;
        }
        if is_prime(i) {
            unsafe {
                STARTER = i + 1;
            }
            return i;
        }
        i += 1;
    }
}

fn power_mod(base:u64, a:u64, p: u64) -> u64 {
    let mut ans:u64 = 1;
    let mut base1:u64 = base;
    let mut a1:u64 = a;
    while a1 != 0 {
        if a1 & 1 == 1 {
            ans = calculate(ans, base1, p);
        };
        base1 = calculate(base1, base1, p);
        a1 = a1 >> 1;
    };
    ans
}

fn calculate(a:u64, b:u64, p:u64) -> u64 {
    let cl = (a as u128) * (b as u128) % (p as u128);
    return cl as u64;
}
