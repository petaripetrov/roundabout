use std::ops::{Add, Deref};

#[derive(Debug)]
pub struct DArray(Vec<f64>);

impl DArray {
    pub fn empty() -> DArray {
        DArray(vec![])
    }
}

impl Deref for DArray {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<f64>> for DArray {
    fn from(value: Vec<f64>) -> Self {
        DArray(value)
    }
}

impl Add for DArray {
    type Output = DArray;

    fn add(self, rhs: Self) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(lhs, rhs)| lhs + rhs)
            .collect::<Vec<f64>>()
            .into()
    }
}

impl PartialEq for DArray {
    fn eq(&self, other: &Self) -> bool {
        0 == self
            .iter()
            .zip(other.iter())
            .filter(|(x, y)| x != y)
            .count()
    }
}

impl PartialEq<Vec<f64>> for DArray {
    fn eq(&self, other: &Vec<f64>) -> bool {
        0 == self
            .iter()
            .zip(other.iter())
            .filter(|(x, y)| x != y)
            .count()
    }
}

impl PartialEq<DArray> for Vec<f64> {
    fn eq(&self, other: &DArray) -> bool {
        0 == self
            .iter()
            .zip(other.iter())
            .filter(|(x, y)| x != y)
            .count()
    }
}
