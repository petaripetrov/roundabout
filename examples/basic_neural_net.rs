extern crate roundabout;

use roundabout::Val;

fn main() {
    // inputs x1, x2
    let x1 = Val::from(2.0);
    let x1_clone = x1.clone();
    let x2 = Val::from(0.0);

    // weights w1, w2
    let w1 = Val::from(-3.0);
    let w2 = Val::from(1.0);

    // bias of the neuron
    let b = Val::from(6.8813735870195432); // nice number for testing 

    // x1*w1 + x2*w2 + b
    let x1w1 = x1*w1;
    let x2w2 = x2*w2;
    let x1w1x2w2 = x1w1 + x2w2;
    let n = x1w1x2w2 + b;
    let o = n.tanh();

    println!("{}", o.data());
    o.backward();
    println!("Grad is: {}", x1_clone.borrow().grad);
}