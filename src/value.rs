// trait Precision {}

// impl Precision for f64 {}
// impl Precision for f32 {}
// impl Precision for f16 {} // not stable

// type PropFn = fn(value: Val);

use std::ops::Add;

#[derive(Debug)] // TODO look itno implementing debug manually
pub struct Val {
    pub data: f64,
    pub grad: f64,

    // prop: PropFn,
    // prev: Vec<Value>,
    // op: String
}

impl Val {
    pub fn new(data: f64) -> Val {
        Val {
            data,
            grad: 0.0,
        }
    }

    fn new_empty() -> Val {
        Val {
            data: 0.0,
            grad: 0.0
        }
    }
}

// TODO fininsh all of the operations
impl Add for Val {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data: self.data + rhs.data,
            grad: 0.0 // TODO put the actual grad here
        }
    }
}

impl PartialEq<f64> for Val{
    fn eq(&self, other: &f64) -> bool {
        self.data == *other
    }
}
