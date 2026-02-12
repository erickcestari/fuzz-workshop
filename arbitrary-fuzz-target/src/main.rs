use honggfuzz::fuzz;
use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
enum Operation {
    Push(i32),
    Pop,
    Peek,
    Clear,
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    initial_capacity: u8,
    operations: Vec<Operation>,
}

fn main() {
    loop {
        fuzz!(|input: FuzzInput| {
            let mut stack: Vec<i32> = Vec::with_capacity(input.initial_capacity as usize);

            for op in &input.operations {
                match op {
                    Operation::Push(val) => {
                        stack.push(*val);
                        assert_eq!(stack.last(), Some(val));
                    }
                    Operation::Pop => {
                        let len_before = stack.len();
                        let popped = stack.pop();
                        if len_before > 0 {
                            assert!(popped.is_some());
                            assert_eq!(stack.len(), len_before - 1);
                        } else {
                            assert!(popped.is_none());
                        }
                    }
                    Operation::Peek => {
                        if !stack.is_empty() {
                            assert!(stack.last().is_some());
                        }
                    }
                    Operation::Clear => {
                        stack.clear();
                        assert!(stack.is_empty());
                    }
                }
            }
        });
    }
}