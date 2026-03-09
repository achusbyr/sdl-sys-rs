//! Generated constants

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    Pointer,
    String,
    Number,
    Float,
    Boolean,
}

#[derive(Debug, Clone, Copy)]
pub struct Hint {
    pub name: &'static str,
    pub value: &'static str,
    pub doc: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Property {
    pub name: &'static str,
    pub value: &'static str,
    pub ty: PropertyType,
    pub doc: &'static str,
}
