use core::u64::MAX;

pub fn collatz(n: u64) -> Option<u64> {
    let mut x = n;
    let mut steps = 0;
    if x == 0 {
        return None;
    }
    while x != 1 {
        x = if x%2==0 {
            x/2
        } else {
          match x.checked_mul(3) {
            Some(y) if y < MAX.try_into().unwrap() => y + 1,
            _ => return None,
        }
        };
        steps+=1;

    }
    Some(steps)

}
