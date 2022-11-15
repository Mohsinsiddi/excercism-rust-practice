use std::collections::BTreeSet;
pub fn sum_of_multiples(limit: u32, numbers: &[u32]) -> u32 {
    let mut multiples: BTreeSet<u32> = BTreeSet::new();
    for &number in numbers {
        let mut multiplier: u32 = 2;
        let mut current = number;
        if current == 0 {continue;}
        while current < limit {
            multiples.insert(current);
            current = number * multiplier;
            multiplier += 1;
        }
    }
    multiples.iter().sum()
}