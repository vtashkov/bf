use std::io::Cursor;
use std::str;

use vtashkov_bf::Interpreter;

#[test]
fn it_can_execute_hello_world() {
    let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut input = Cursor::new(vec![]);
    let mut output = vec![];
    let mut interpreter = Interpreter::new(&mut input, &mut output, 30000);
    interpreter.execute(&source_code);
    assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
}
