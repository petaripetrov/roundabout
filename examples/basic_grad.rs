use roundabout::Val;

fn main() {
    let a = Val::from(-4.0);
    let a_c = a.clone();
    let b = Val::from(2.0);
    let b_c = b.clone();

    let c = a.clone() + b.clone();
    let d = a.clone() * b.clone() + b.pow(Val::from(3));

    let c_1 = (c * 2.into()) + Val::from(1);
    let c_2 = (c_1 * 2.into()) + Val::from(1) + -a.clone();

    let d_1 = d.clone() + (d * Val::from(2)) + (b.clone() + a.clone()).relu();
    let d_2 = d_1.clone() + (d_1 * Val::from(3)) + (b - a).relu();

    let e = c_2 - d_2;
    let f = e.pow(2.into());
    let g = f.clone() / 2.into();
    let g_1 = g + (Val::from(10.0) / f);

    println!("{:.4}", g_1.data());
    g_1.backward();
    println!(
        "a grad: {:.4} \nb grad: {:.4}",
        a_c.borrow().grad,
        b_c.borrow().grad
    );
}
