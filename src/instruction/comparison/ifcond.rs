use crate::instruction::instruction::ExecuteResult;
use crate::rtda::frame::Frame;
use crate::rtda::thread::Thread;
use crate::util::code_reader::CodeReader;

fn _ifcond(frame: Frame) -> (i32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (val, operand_stack) = operand_stack.pop_int();
    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    (val, frame)
}

#[allow(non_snake_case)]
pub fn IFEQ(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFEQ");
    let (offset, code_reader) = code_reader.read_i16();
    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val == 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFNE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFNE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val != 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFLT(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFLT");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val < 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFGE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFGE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val >= 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFGT(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFGT");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val > 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFLE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IFLE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val, frame) = _ifcond(frame);
    let offset = if val <= 0 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use vec_map::VecMap;

    use crate::classfile::constant_pool::ConstantPool;
    use crate::classfile::member_info::MemberInfo;
    use crate::instruction::comparison::ifcond::*;
    use crate::instruction::comparison::ifcond::IFLT;
    use crate::instruction::instruction::ExecuteResult;
    use crate::rtda::frame::Frame;
    use crate::rtda::heap::class::Class;
    use crate::rtda::heap::method::Method;
    use crate::rtda::thread::Thread;
    use crate::rtda::vars::Vars;
    use crate::util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_IFEQ_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFEQ(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFEQ_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFEQ(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFNE_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFNE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFNE_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFNE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFLT_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFLT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFLT_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFLT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFGE_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFGE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFGE_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFGE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFGT_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFGT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFGT_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFGT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFLE_success() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFLE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFLE_fail() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IFLE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }
}
