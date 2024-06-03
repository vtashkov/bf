use std::io::Cursor;
use std::str;

use vtashkov_bf::Interpreter;
use vtashkov_bf::Program;

#[test]
fn it_can_execute_hello_world() {
    let mut input = Cursor::new(vec![]);
    let mut output = vec![];
    let mut interpreter = Interpreter::new(&mut input, &mut output, 30000);
    let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let program = Program::parse(source_code);
    interpreter.execute(program);
    assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
}
