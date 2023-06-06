use crate::instruction::instruction::ExecuteResult;
use crate::rtda::frame::Frame;
use crate::rtda::thread::Thread;
use crate::util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DNEG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_double();
    let operand_stack = operand_stack.push_double(-v);
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

#[allow(non_snake_case)]
pub fn FNEG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_float();
    let operand_stack = operand_stack.push_float(-v);
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

#[allow(non_snake_case)]
pub fn INEG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_int();
    let operand_stack = operand_stack.push_int(-v);
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

#[allow(non_snake_case)]
pub fn LNEG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_long();
    let operand_stack = operand_stack.push_long(-v);
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

#[cfg(test)]
mod tests {
    use std::f32;
    use std::f64;
    use std::rc::Rc;

    use vec_map::VecMap;

    use crate::classfile::constant_pool::ConstantPool;
    use crate::classfile::member_info::MemberInfo;
    use crate::instruction::instruction::ExecuteResult;
    use crate::instruction::math::neg::*;
    use crate::instruction::math::neg::*;
    use crate::instruction::math::neg::FNEG;
    use crate::instruction::math::neg::INEG;
    use crate::instruction::math::neg::LNEG;
    use crate::rtda::frame::Frame;
    use crate::rtda::heap::class::Class;
    use crate::rtda::heap::method::Method;
    use crate::rtda::thread::Thread;
    use crate::rtda::vars::Vars;
    use crate::util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG() {
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

        let operand_stack = operand_stack.push_double(2f64);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, -2f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG_zero() {
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

        let operand_stack = operand_stack.push_double(-0f64);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 0f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG_inf() {
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

        let operand_stack = operand_stack.push_double(f64::INFINITY);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, f64::NEG_INFINITY);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FNEG() {
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

        let operand_stack = operand_stack.push_float(-100.7678f32);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 100.7678f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FNEG_max_min() {
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

        let operand_stack = operand_stack.push_float(f32::MAX);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, f32::MIN);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_INEG() {
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

        let operand_stack = operand_stack.push_int(234556);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            INEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -234556);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LNEG() {
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

        let operand_stack = operand_stack.push_long(-54875845748435i64);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LNEG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 54875845748435i64);
    }
}
