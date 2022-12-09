#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        if match input {
            CalculatorInput::Add => operator(&mut stack, |a, b| a + b),
            CalculatorInput::Subtract => operator(&mut stack, |a, b| a - b),
            CalculatorInput::Multiply => operator(&mut stack, |a, b| a * b),
            CalculatorInput::Divide => operator(&mut stack, |a, b| a / b),
            CalculatorInput::Value(n) => {
                stack.push(*n);
                Ok(())
            }
        }
        .is_err()
        {
            return None;
        }
    }
    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}
fn operator<F>(stack: &mut Vec<i32>, op: F) -> Result<(), ()>
where
    F: Fn(i32, i32) -> i32,
{
    if stack.len() < 2 {
        return Err(());
    }
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(op(b, a));
    Ok(())
}
