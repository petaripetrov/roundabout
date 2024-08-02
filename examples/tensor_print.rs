use roundabout::Tensor;

fn main() {
    let x = Tensor::new(vec![1.0, 1.0, 0.0, 1.0, 1.0, 1.0], (2, 3));

    println!("{}", x);
}