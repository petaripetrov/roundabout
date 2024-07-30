use std::ops::{Add, Div, Mul, Sub};

use crate::{val::{Data, PropFn}, Val};

// TODO either make an engine object (that implements these)
// or put these on the actual Val object (i.e. _add(self, rhs))
impl Add for Val {
    type Output = Val;

    fn add(self, rhs: Self) -> Self::Output {

        let prop_fn: PropFn = |value| {
            for el in value.prev.iter() {
                // Have to iterate over each element to make sure we do not
                // borrow the same Val twice (i.e. b = a + a)
                let mut borrowed = el.borrow_mut();
                borrowed.grad += value.grad;
            }
        };

        Val::new(Data {
            data: self.borrow().data + rhs.borrow().data,
            grad: 0.0,
            prev: vec![self.clone(), rhs.clone()],
            prop: Some(prop_fn),
            op: Some("+".to_owned()),
            label: None,
        })
    }
}

impl Sub for Val {
    type Output = Val;

    fn sub(self, rhs: Self) -> Self::Output {
        Val::new(Data {
            data: self.borrow().data - rhs.borrow().data,
            grad: 0.0,
            prev: vec![self.clone(), rhs.clone()],
            prop: None,
            op: Some("-".to_owned()),
            label: None,
        })
    }
}

impl Mul for Val {
    type Output = Val;

    fn mul(self, rhs: Self) -> Self::Output {

        let prop_fn: PropFn = |value| {
            // Ditto Add
            let mut data: Vec<f64> = value.prev.iter().map(|v| v.borrow().data).collect();

            for el in value.prev.iter() {
                let mut borrowed = el.borrow_mut();

                borrowed.grad += data.pop().unwrap() * value.grad;
            }
        };

        Val::new(Data {
            data: self.borrow().data * rhs.borrow().data,
            grad: 0.0,
            prev: vec![self.clone(), rhs.clone()],
            prop: Some(prop_fn),
            op: Some("-".to_owned()),
            label: None,
        })
    }
}

impl Div for Val {
    type Output = Val;

    fn div(self, rhs: Self) -> Self::Output {
        Val::new(Data {
            data: self.borrow().data / rhs.borrow().data,
            grad: 0.0,
            prev: vec![self.clone(), rhs.clone()],
            prop: None,
            op: Some("-".to_owned()),
            label: None,
        })
    }
}
