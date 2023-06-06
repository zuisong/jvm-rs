use crate::instruction::instruction::ExecuteResult;
use crate::rtda::frame::*;
use crate::rtda::thread::Thread;
use crate::util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _fcmp(frame: Frame, flag: bool) -> (f32, f32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_float();
    let (val1, operand_stack) = operand_stack.pop_float();

    let operand_stack = if val1 < val2 {
        operand_stack.push_int(-1)
    } else if val1 > val2 {
        operand_stack.push_int(1)
    } else if val1 == val2 {
        operand_stack.push_int(0)
    } else if flag {
        operand_stack.push_int(1)
    } else {
        operand_stack.push_int(-1)
    };

    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn FCMPG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("FCMPG");

    let (frame, thread) = thread.pop_frame();
    let (_, _, frame) = _fcmp(frame, true);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FCMPL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("FCMPL");

    let (frame, thread) = thread.pop_frame();
    let (_, _, frame) = _fcmp(frame, false);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use vec_map::VecMap;

    use crate::classfile::constant_pool::ConstantPool;
    use crate::classfile::member_info::MemberInfo;
    use crate::instruction::comparison::fcmp::*;
    use crate::instruction::comparison::fcmp::{FCMPG, FCMPL};
    use crate::instruction::instruction::ExecuteResult;
    use crate::rtda::frame::Frame;
    use crate::rtda::heap::class::Class;
    use crate::rtda::heap::method::Method;
    use crate::rtda::operand_stack::OperandStack;
    use crate::rtda::thread::Thread;
    use crate::rtda::vars::Vars;
    use crate::util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPL() {
        let frame = create_frame(0.03, 0.042);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FCMPL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPG() {
        let frame = create_frame(1.21, 1.1);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FCMPG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPG_equal() {
        let frame = create_frame(2.345, 2.345);
        let thread = Thread::new().push_frame(frame);

        let (ExecuteResult { thread, offset: _ }, _) =
            FCMPG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    fn create_frame(op1: f32, op2: f32) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = operand_stack.push_float(op1);
        let operand_stack = operand_stack.push_float(op2);
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: vec![],
        }));
        let class = Rc::new(Class {
            access_flags: 0u16,
            name: "".to_string(),
            constant_pool: ConstantPool {
                vec_map: VecMap::new(),
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            instance_slot_count: 0usize,
            static_slot_count: 0usize,
            static_vars: Vars::new(2),
        });
        Frame {
            class,
            local_vars: Vars::new(10),
            operand_stack: operand_stack,
            method,
        }
    }
}
