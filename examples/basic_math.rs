extern crate roundabout;

use roundabout::Val;

fn main() {
    let a = Val::from(2.0);
    let a_clone = a.clone();
    let b = Val::from(-3.0);
    let c = Val::from(10.0);
    let e = a * b;
    let d = e + c;
    let f = Val::from(-2.0);
    let l = d * f;

    println!("{}", l.data());
    l.backward();
    println!("{}", a_clone.borrow().grad)
}
