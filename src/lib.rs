mod val;
mod engine;
mod tensor;

pub use crate::val::Val;
pub use crate::tensor::Tensor;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use val::Val;

    #[test]
    fn creation_works() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);
        
        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x: Val = raw_x.into();
        let y: Val = raw_y.into();

        assert_eq!(x.data(), raw_x);
        assert_eq!(y.data(), raw_y)
    }

    #[test]
    fn addition_works() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x: Val = raw_x.into();
        let y: Val = raw_y.into();

        let result = x + y;
        assert_eq!(result, raw_x + raw_y);
    }

    #[test]
    fn subtraction_works() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x: Val = raw_x.into();
        let y: Val = raw_y.into();

        let result = x - y;
        assert_eq!(result, raw_x - raw_y);
    }

    #[test]
    fn multiplication_works() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x: Val = raw_x.into();
        let y: Val = raw_y.into();

        let result = x * y;
        assert_eq!(result, raw_x * raw_y);
    }

    #[test]
    fn division_works() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x: Val = raw_x.into();
        let y: Val = raw_y.into();

        let result = x / y;
        assert_eq!(result, raw_x / raw_y);
    }

    #[test]
    fn addition_child() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let raw_x = rng.gen::<f64>();
        let raw_y = rng.gen::<f64>();

        let x = Val::from(raw_x);
        let y = Val::from(raw_y);

        let result = x.clone() + y.clone();
        let prev = result.borrow().prev.clone();
        assert_eq!(result, raw_x + raw_y);
        assert_eq!(prev, vec![x, y])
    }
}
