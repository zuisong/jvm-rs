use crate::rtda::heap::class::Class;
use crate::rtda::slot::Slot;

struct Object {
    class: Class,
    fields: Vec<Slot>,
}
