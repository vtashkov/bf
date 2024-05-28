pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { }
    }

    pub fn execute(&self, _source_code: &str) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_interpreter() {
        let _interpreter = Interpreter::new();
    }

    #[test]
    fn can_execute_code() {
        let interpreter = Interpreter::new();
        interpreter.execute("");
    }
}
