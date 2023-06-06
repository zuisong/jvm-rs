use crate::instruction::instruction::ExecuteResult;
use crate::rtda::frame::Frame;
use crate::rtda::thread::Thread;
use crate::util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn BIPUSH(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("BIPUSH");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (val, code_reader) = code_reader.read_i8();
    let i = val;
    let operand_stack = operand_stack.push_int(i as i32);

    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
