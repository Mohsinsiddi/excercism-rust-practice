pub fn brackets_are_balanced(string: &str) -> bool {
    let mut char_stack = Vec::<char>::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => char_stack.push(c),
            ')' => if char_stack.pop()!=Some('(') {return  false},
            ']' => if char_stack.pop()!=Some('[') {return  false},
            '}' => if char_stack.pop()!=Some('{') {return  false},
            _ =>()
        }
    }
    char_stack.is_empty()
}
