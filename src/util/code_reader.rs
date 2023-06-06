extern crate byteorder;

use std;
use std::rc::Rc;

use self::byteorder::{BigEndian, ByteOrder};

pub struct CodeReader {
    code: Rc<Vec<u8>>,
    pub pc: usize,
}

impl CodeReader {
    pub fn new(code: Rc<Vec<u8>>) -> CodeReader {
        CodeReader { code, pc: 0 }
    }

    pub fn set_pc(self, pc: usize) -> CodeReader {
        let CodeReader { pc: _, code } = self;
        CodeReader { code, pc }
    }

    pub fn read_u8(self) -> (u8, CodeReader) {
        let CodeReader { pc, code } = self;
        let val = code[pc];
        let pc = pc + 1;
        let code_reader = CodeReader { pc, code };
        (val, code_reader)
    }

    pub fn read_i8(self) -> (i8, CodeReader) {
        let CodeReader { pc, code } = self;
        let v = code[pc];
        let val = unsafe { std::mem::transmute::<u8, i8>(v) };
        let pc = pc + 1;
        let code_reader = CodeReader { pc, code };
        (val, code_reader)
    }

    pub fn read_u16(self) -> (u16, CodeReader) {
        let CodeReader { pc, code } = self;
        let val = {
            let seq = &code[pc..(pc + 2)];
            BigEndian::read_u16(&seq)
        };
        let pc = pc + 2;
        let code_reader = CodeReader { pc, code };
        (val, code_reader)
    }

    pub fn read_i16(self) -> (i16, CodeReader) {
        let CodeReader { pc, code } = self;

        let val = {
            let seq = &code[pc..(pc + 2)];
            BigEndian::read_i16(&seq)
        };
        let pc = pc + 2;
        let code_reader = CodeReader { pc, code };
        (val, code_reader)
    }
}
