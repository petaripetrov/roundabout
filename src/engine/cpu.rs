use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Val;

// engine
fn add(lhs: &Val, rhs: &Val) -> Val {
    Val {
        data: lhs.data + rhs.data,
        grad: 0.0,
        prev: vec![lhs.clone(), rhs.clone()],
        op: Some("+".to_owned()),
        label: Some(format!("{:?} + {:?}", lhs.label, rhs.label))
    }
}

fn sub(lhs: &Val, rhs: &Val) -> Val {
    Val {
        data: lhs.data - rhs.data,
        grad: 0.0,
        prev: vec![lhs.clone(), rhs.clone()],
        op: Some("-".to_owned()),
        label: Some(format!("{:?} - {:?}", lhs.label, rhs.label))
    }
}

fn mul(lhs: &Val, rhs: &Val) -> Val {
    Val {
        data: lhs.data * rhs.data,
        grad: 0.0,
        prev: vec![lhs.clone(), rhs.clone()],
        op: Some("*".to_owned()),
        label: Some(format!("{:?} * {:?}", lhs.label, rhs.label))
    }
}

fn div(lhs: &Val, rhs: &Val) -> Val {
    Val {
        data: lhs.data / rhs.data,
        grad: 0.0,
        prev: vec![lhs.clone(), rhs.clone()],
        op: Some("/".to_owned()),
        label: Some(format!("{:?} / {:?}", lhs.label, rhs.label))
    }
}

// operator implementations

impl Add for Val {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        add(&self, &rhs)
    }
}

impl<'a, 'b> Add<&'b Val> for &'a Val {
    type Output = Val;

    fn add(self, rhs: &'b Val) -> Self::Output {
        add(&self, rhs)
    }
}

impl<'a> Add<f64> for &'a Val {
    type Output = Val;

    fn add(self, rhs: f64) -> Self::Output {
        add(&self, &rhs.into())
    }
}

impl<'a> Add<&'a Val> for f64 {
    type Output = Val;

    fn add(self, rhs: &Val) -> Self::Output {
        add(&self.into(), &rhs)
    }
}

impl Add<f64> for Val {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        add(&self, &rhs.into())
    }
}

impl Add<Val> for f64 {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        add(&self.into(), &rhs)
    }
}

impl Sub for Val {
    // TODO add implementations for f64 and f64 - val
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        sub(&self, &rhs)
    }
}

impl<'a, 'b> Sub<&'b Val> for &'a Val {
    type Output = Val;

    fn sub(self, rhs: &'b Val) -> Self::Output {
        sub(self, rhs)
    }
}

impl Mul for Val {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        mul(&self, &rhs)
    }
}

impl<'a, 'b> Mul<&'b Val> for &'a Val {
    type Output = Val;

    fn mul(self, rhs: &'b Val) -> Self::Output {
        mul(self, rhs)
    }
}

impl<'a> Mul<f64> for &'a Val {
    type Output = Val;

    fn mul(self, rhs: f64) -> Self::Output {
        mul(self, &rhs.into())
    }
}

impl<'a> Mul<&'a Val> for f64 {
    type Output = Val;

    fn mul(self, rhs: &'a Val) -> Self::Output {
        mul(&self.into(), rhs)
    }
}

impl<'a, 'b> Div<&'b Val> for &'a Val {
    type Output = Val;

    fn div(self, rhs: &'b Val) -> Self::Output {
        div(self, rhs)
    }
}

impl<'a> Div<f64> for &'a Val {
    type Output = Val;

    fn div(self, rhs: f64) -> Self::Output {
        div(self, &rhs.into())
    }
}

impl<'a> Div<&'a Val> for f64 {
    type Output = Val;

    fn div(self, rhs: &'a Val) -> Self::Output {
        div(&self.into(), rhs)
    }
}

impl<'a> Neg for &'a Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        self * -1.0
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
