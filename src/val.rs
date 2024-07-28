use std::{cell::RefCell, hash::Hash, ops::Deref, rc::Rc};

pub type PropFn = fn(value: &mut Data);

#[derive(Debug, Clone)]
pub struct Data {
    pub data: f64,
    pub grad: f64,
    pub prev: Vec<Val>,
    pub prop: Option<PropFn>,
    pub op: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Val(Rc<RefCell<Data>>);

impl Val {
    pub fn new(data: Data) -> Val {
        Val(Rc::new(RefCell::new(data)))
    }

    pub fn data(&self) -> f64 {
        self.0.borrow().data
    }
}

// Type traits
impl Deref for Val {
    type Target = Rc<RefCell<Data>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<f64>> From<T> for Val {
    fn from(value: T) -> Self {
        let data  = Data {
            data: value.into(),
            grad: 0.0,
            prev: vec![],
            prop: None,
            op: None,
            label: None,
        };

        Val::new(data)
    }
}

impl PartialEq<f64> for Val {
    fn eq(&self, other: &f64) -> bool {
        self.0.borrow().data == *other
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        // TODO expand this to check more things
        self.data == other.data
    }
}

impl PartialEq<f64> for Data {
    fn eq(&self, other: &f64) -> bool {
        self.data == *other
    }
}

impl Eq for Data {}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.grad.to_bits().hash(state);
        self.prev.hash(state);
        self.prop.hash(state);
        self.op.hash(state);
        self.label.hash(state);
    }
}

impl Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}