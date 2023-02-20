use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut setMap = HashSet::new();

    let lowercase_chars = candidate.to_lowercase();

    let chars = lowercase_chars.chars().filter(|x| *x!=' ' && *x !='-');
     
    for c in chars {
        if !setMap.insert(c) {
            return  false;
        }
    }
    return  true;
}
