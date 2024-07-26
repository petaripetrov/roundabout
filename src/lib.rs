mod value;

#[cfg(test)]
mod tests {
    use super::*;
    use value::Val;

    #[test]
    fn it_works() {
        let x = Val::new(2.0);
        let y = Val::new(2.0);
        let result = x + y;
        assert_eq!(result, 4.0);
    }
}
