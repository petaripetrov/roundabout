use std::ops::{Add, Div, Mul, Sub};

use crate::{val::{Data, PropFn}, Val};

// TODO either make an engine object (that implements these)
// or put these on the actual Val object (i.e. _add(self, rhs))
impl Add for Val {
    type Output = Val;

    fn add(self, rhs: Self) -> Self::Output {

        let prop_fn: PropFn = |value| {
            let mut first = value.prev[0].borrow_mut();
            let mut second = value.prev[1].borrow_mut();
            
            println!("{}", value.grad);

            first.grad += value.grad;
            second.grad += value.grad;

            println!("{}", second.grad);
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
            let mut first = value.prev[0].borrow_mut();
            let mut second = value.prev[1].borrow_mut();

            first.grad += second.data * value.grad;
            second.grad += first.data * value.grad;
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
