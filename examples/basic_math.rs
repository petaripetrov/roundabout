extern crate roundabout;

use roundabout::Val;

fn main() {
    let a = &Val::new(-4.0);
    let b = &Val::new(2.0);

    let c = &(a + b);
    let d = &(a * b + b.pow(3.into()));

    let c_1 = c + &(c + 1.0);
    let c_2 = &c_1 + &(1.0 + &c_1 + (-a));
    let d_1 = d + &(d * 2.0 + (b + a).relu());
    let d_2 = &d_1 + &(3.0 * &d_1 + (b - a).relu());

    let e = c_2 - d_2;
    let f = &e.pow(2.0.into());
    let g = f / 2.0;

    let g_1 = g + 10.0 / f;

    print!("{:?}", g_1);
    // g.backward();
}
