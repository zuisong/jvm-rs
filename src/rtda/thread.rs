use crate::rtda::frame::Frame;
use crate::rtda::stack::Stack;

const STACK_SIZE: usize = 1024;

pub struct Thread {
    stack: Stack,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            stack: Stack::new(STACK_SIZE),
        }
    }

    pub fn push_frame(self, frame: Frame) -> Thread {
        let Thread { stack } = self;
        Thread {
            stack: stack.push(frame),
        }
    }

    pub fn pop_frame(self) -> (Frame, Thread) {
        let Thread { stack } = self;
        let (frame, stack) = stack.pop();
        let thread = Thread { stack };
        (frame, thread)
    }

    pub fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
