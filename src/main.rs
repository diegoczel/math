use fraction::Fraction;
use crate::fraction::{lcm, FractionMixed};

pub mod fraction;
pub mod prime;

fn main() {
    let mut x = [5, 2];
    println!("lcm: {}", lcm(&mut x));
    println!();

    let mut f1 = Fraction {n: 10, d: 15};
    println!("F begin apply lcm: {:?}", f1);
    f1.apply_lcm(180);
    println!("F after apply lcm: {:?}", f1);
    println!();

    let mut f2 = Fraction {n: 10, d: 15};
    println!("F begin apply lcm: {:?}", f2);
    f2.simplify();
    println!("F after apply lcm: {:?}", f2);
    println!();

    let mut f3 = Fraction {n: 3, d: 15};
    let f4 = Fraction {n: 7, d: 15};
    f3.sum(&f4);
    println!("sum: {:?}", f3);
    f3.simplify();
    println!("simplify: {:?}", f3);
    println!();

    let mut f5 = Fraction {n: 8, d: 18};
    let f6 = Fraction {n: 5, d: 18};
    f5.sub(&f6);
    println!("sub: {:?}", f5);
    f5.simplify();
    println!("simplify: {:?}", f5);
    println!();

    println!("test Fraction -> FractionMixed");
    let f_imp1 = FractionMixed{num_improprio: 5, fracao: Fraction {n: 1, d: 4}};
    let f7 = f_imp1.get_fraction();
    println!("{:?}", f_imp1);
    println!("{:?}", f7);
    println!();

    println!("test FractionMixed -> Fraction");
    let f8 = Fraction {n: 7, d: 4};
    let f_imp2 = f7.get_fraction_mixed();
    let f_imp3 = f8.get_fraction_mixed();
    println!("{:?}", f_imp2);
    println!("{:?}", f_imp3);
    println!();

}
