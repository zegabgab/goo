use std::collections::HashMap;

#[derive(Default)]
pub struct Interpreter {
    vm: VM,
}

pub enum InterpretResult {
    Success,
    CompileError(String),
    RuntimeError(String),
}

#[derive(Default)]
struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

type Value = f64;

impl Interpreter {
    pub fn new() -> Interpreter {
        Self::default()
    }

    pub fn execute(&mut self, source: String) -> InterpretResult {
        println!("Interpreting {source}");
        InterpretResult::Success
    }
}
