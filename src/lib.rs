mod val;
mod engine;

pub use crate::val::{Val};

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

    // #[test]
    // fn addition_child() {
    //     let mut rng: StdRng = StdRng::seed_from_u64(42);

    //     let raw_x = rng.gen::<f64>();
    //     let raw_y = rng.gen::<f64>();

    //     let x = Val::new(raw_x);
    //     let y = Val::new(raw_y);

    //     let result = &x + &y;
    //     assert_eq!(result, raw_x + raw_y);
    //     assert_eq!(result.prev, vec![x, y])
    // }

    // #[test]
    // fn subtraction_child() {
    //     let mut rng: StdRng = StdRng::seed_from_u64(42);

    //     let raw_x = rng.gen::<f64>();
    //     let raw_y = rng.gen::<f64>();

    //     let x = Val::new(raw_x);
    //     let y = Val::new(raw_y);

    //     let result = &x - &y;
    //     assert_eq!(result, raw_x - raw_y);
    //     assert_eq!(result.prev, vec![x, y])
    // }

    // #[test]
    // fn multiplication_child() {
    //     let mut rng: StdRng = StdRng::seed_from_u64(42);

    //     let raw_x = rng.gen::<f64>();
    //     let raw_y = rng.gen::<f64>();

    //     let x = Val::new(raw_x);
    //     let y = Val::new(raw_y);

    //     let result = &x * &y;
    //     assert_eq!(result, raw_x * raw_y);
    //     assert_eq!(result.prev, vec![x, y])

    // }

    // #[test]
    // fn division_child() {
    //     let mut rng: StdRng = StdRng::seed_from_u64(42);

    //     let raw_x = rng.gen::<f64>();
    //     let raw_y = rng.gen::<f64>();

    //     let x = Val::new(raw_x);
    //     let y = Val::new(raw_y);

    //     let result = &x / &y;
    //     assert_eq!(result, raw_x / raw_y);
    //     assert_eq!(result.prev, vec![x, y])
    // }
}
