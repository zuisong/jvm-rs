use std::rc::Rc;

use crate::classfile::constant_info::ConstantInfo;
use crate::classfile::constant_pool::ConstantPool;
use crate::rtda::heap::symbol_ref::SymbolRef;

struct ClassRef {
    symbol_ref: SymbolRef,
}

impl ClassRef {
    fn new(constant_pool: Rc<ConstantPool>, constant_class_info: ConstantInfo) -> ClassRef {
        let symbol_ref = SymbolRef{
            constant_pool,

        }
    }
}
