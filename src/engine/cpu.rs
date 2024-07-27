use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Val;

impl Add for Val {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let data = self.data + rhs.data;

        Val::new(data).with_children(vec![self, rhs]).with_op("+")
    }
}

impl<'a, 'b> Add<&'b Val> for &'a Val {
    type Output = Val;

    fn add(self, rhs: &'b Val) -> Self::Output {
        let data = self.data + rhs.data;

        Val::new(data).with_children(vec![self.clone(), rhs.clone()]).with_op("+")
    }
}

impl<'a> Add<f64> for &'a Val {
    type Output = Val;

    fn add(self, rhs: f64) -> Self::Output {
        let data = self.data + rhs;

        Val::new(data).with_children(vec![self.clone(), Val::from(rhs)]).with_op("+")
    }
}

impl<'a> Add<&'a Val> for f64 {
    type Output = Val;

    fn add(self, rhs: &Val) -> Self::Output {
        let data = self + rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs.clone()]).with_op("+")
    }
}

impl Add<f64> for Val {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let data = self.data + rhs;

        Val::new(data).with_children(vec![self, Val::from(rhs)]).with_op("+")
    }
}

impl Add<Val> for f64 {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        let data = self + rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs]).with_op("+")
    }
}

impl Sub for Val {
    // TODO add implementations for f64 and f64 - val
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let data = self.data - rhs.data;

        Val::new(data).with_children(vec![self, rhs]).with_op("-")
    }
}

impl<'a, 'b> Sub<&'b Val> for &'a Val {
    type Output = Val;

    fn sub(self, rhs: &'b Val) -> Self::Output {
        let data = self.data - rhs.data;

        Val::new(data).with_children(vec![self.clone(), rhs.clone()]).with_op("-")
    }
}

impl<'a, 'b> Sub<&'b mut Val> for &'a mut Val {
    type Output = Val;

    fn sub(self, rhs: &'b mut Val) -> Self::Output {
        let data = self.data - rhs.data;

        Val::new(data).with_children(vec![self.clone(), rhs.clone()]).with_op("-")
    }
}

impl Mul for Val {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = self.data * rhs.data;

        Val::new(data).with_children(vec![self, rhs]).with_op("*")
    }
}

impl<'a, 'b> Mul<&'b Val> for &'a Val {
    type Output = Val;

    fn mul(self, rhs: &'b Val) -> Self::Output {
        let data = self.data * rhs.data;

        Val::new(data).with_children(vec![self.clone(), rhs.clone()]).with_op("*")
    }
}

impl<'a> Mul<f64> for &'a Val {
    type Output = Val;

    fn mul(self, rhs: f64) -> Self::Output {
        let data = self.data * rhs;

        Val::new(data).with_children(vec![self.clone(), Val::from(rhs)]).with_op("*")
    }
}

impl<'a> Mul<&'a Val> for f64 {
    type Output = Val;

    fn mul(self, rhs: &'a Val) -> Self::Output {
        let data = self * rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs.clone()]).with_op("*")
    }
}

impl Mul<f64> for Val {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let data = self.data * rhs;

        Val::new(data).with_children(vec![self, Val::from(rhs)]).with_op("*")
    }
}

impl Mul<Val> for f64 {
    type Output = Val;

    fn mul(self, rhs: Val) -> Self::Output {
        let data = self * rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs]).with_op("*")
    }
}

impl Div for Val {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data = self.data / rhs.data;

        Val::new(data).with_children(vec![self, rhs]).with_op("/")
    }
}

impl<'a, 'b> Div<&'b Val> for &'a Val {
    type Output = Val;

    fn div(self, rhs: &'b Val) -> Self::Output {
        let data = self.data / rhs.data;

        Val::new(data).with_children(vec![self.clone(), rhs.clone()]).with_op("/")
    }
}

impl Div<f64> for Val {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let data = self.data / rhs;

        Val::new(data).with_children(vec![self, Val::from(rhs)]).with_op("/")
    }
}

impl Div<Val> for f64 {
    type Output = Val;

    fn div(self, rhs: Val) -> Self::Output {
        let data = self / rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs]).with_op("/")
    }
}

impl<'a> Div<f64> for &'a Val {
    type Output = Val;

    fn div(self, rhs: f64) -> Self::Output {
        let data = self.data / rhs;

        Val::new(data).with_children(vec![self.clone(), Val::from(rhs)]).with_op("/")
    }
}

impl<'a> Div<&'a Val> for f64 {
    type Output = Val;

    fn div(self, rhs: &'a Val) -> Self::Output {
        let data = self / rhs.data;

        Val::new(data).with_children(vec![Val::from(self), rhs.clone()]).with_op("/")
    }
}

impl Neg for Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        Val::new(-self.data).with_children(vec![self])
    }
}

impl<'a> Neg for &'a Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        Val::new(-self.data).with_children(vec![self.clone()])
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl PartialEq<f64> for Val {
    fn eq(&self, other: &f64) -> bool {
        self.data == *other
    }
}
