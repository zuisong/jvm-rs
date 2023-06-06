use crate::instruction::instruction::ExecuteResult;
use crate::rtda::thread::Thread;
use crate::util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn NOP(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
