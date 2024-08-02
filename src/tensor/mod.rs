use core::fmt;
use std::ops::Add;

use vec_ops::DArray;

mod vec_ops;

#[derive(Debug)]
pub struct Tensor {
    pub data: DArray,
    pub shape: (usize, usize),
}

impl Tensor {
    pub fn new(data: Vec<f64>, shape: (usize, usize)) -> Self {
        let (x, y) = shape;

        if data.len() != x * y {
            panic!("Data does not match specified shape.")
        }

        Self {
            data: data.into(),
            shape,
        }
    }
}

impl fmt::Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO replace unwraps here with actual error handling
        write!(f, "Tensor(").unwrap();
        let (x, y) = self.shape;

        write!(f, "data=[").unwrap();
        for row in 0..x {
            write!(f, "[").unwrap();
            for col in 0..y {
                let delim = if col != y - 1 {","} else {""};
                write!(f, "{}{delim}", self.data[y * row + col]).unwrap();
            }
            write!(f, "]").unwrap();
        }
        write!(f, "]").unwrap();

        write!(f, ", shape=({}, {}))", x, y)
    }
}

impl Add for Tensor {
    type Output = Tensor;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO Make sure shape is valid here

        // make a macro for this (that throws a compiler error)
        if self.shape != rhs.shape {
            panic!("Shapes do not match");
        }

        Tensor {
            data: self.data + rhs.data,
            shape: self.shape,
        }
    }
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.shape == other.shape
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::Tensor;

    #[test]
    fn add_works() {
        let x = Tensor::new(vec![1.0, 0.0, 0.0], (1, 3));
        let y = Tensor::new(vec![0.0, 1.0, 0.0], (1, 3));

        let z = x + y;
        assert_eq!(Tensor::new(vec![1.0, 1.0, 0.0], (1, 3)), z)
    }

    #[test]
    #[should_panic]
    fn data_shape_match() {
        let _ = Tensor::new(vec![1.0, 0.0, 0.0], (1, 10));
    }

    #[test]
    #[should_panic]
    fn add_shapes_match() {
        let x = Tensor::new(vec![1.0, 0.0, 0.0], (1, 3));
        let y = Tensor::new(vec![0.0, 1.0], (1, 2));

        let _ = x + y;
    }

    #[test]
    fn to_string() {
        let x = Tensor::new(vec![1.0, 1.0, 0.0, 1.0, 1.0, 1.0], (2, 3));

        let string = format!("{}", x);

        assert_eq!(string, "Tensor(data=[[1,1,0][1,1,1]], shape=(2, 3))")
    }
}
