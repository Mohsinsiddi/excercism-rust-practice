pub fn factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut rem = n;

    for i in 2..=n{
        while rem%i==0 {
            v.push(i);
            rem/=i;
        }
        if rem ==1 {
            return v;
        }
    }
    v
}
