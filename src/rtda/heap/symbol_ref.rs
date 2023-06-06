use std::rc::Rc;

use crate::classfile::constant_info::ConstantInfo;
use crate::classfile::constant_pool::ConstantPool;
use crate::rtda::heap::class::Class;

pub struct SymbolRef {
    constant_pool: Rc<ConstantPool>,
    class_name: String,
    class: Rc<Class>,
}

impl SymbolRef {
    fn new(constant_pool: Rc<ConstantPool>, constant_class_info: ConstantInfo) -> ClassRef {
        let class_name =
    }
}
