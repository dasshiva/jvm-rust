pub struct Ref<T> {
    id: u32,
    reftype: RefType,
    data: Box<T>,
}

enum RefType {
    Class,
    Array,
}
