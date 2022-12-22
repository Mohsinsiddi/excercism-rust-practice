pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = array.len();
    let mut mid = end / 2;
    
    while start != end {
        use std::cmp::Ordering::*;
        match key.cmp(&array[mid]) {
            Less => end = mid,
            Equal => return Some(mid),
            Greater => start = mid + 1,
        }
        mid = start + (end - start) / 2;
    }
    None
}
