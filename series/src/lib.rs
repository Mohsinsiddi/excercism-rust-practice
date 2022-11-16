pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    if digits.len() < len {
        return result;
    }
    for i in 0..(digits.len()-len+1) {
        let x = String::from(&digits[i..i+len]);
        result.push(x);
    }
    result
}
