use crate::prime::{is_prime, next_prime};

#[derive(PartialEq, Debug)]
pub struct FractionMixed {
    pub num_improprio: i32,
    pub fracao: Fraction,
}

impl FractionMixed {
    pub fn get_fraction(&self) -> Fraction {
        Fraction {n: (self.fracao.d * self.num_improprio) + self.fracao.n, d: self.fracao.d}
    }

    pub fn sum(&mut self, other: &FractionMixed) {
        self.fracao.sum(&other.fracao);

        if self.fracao.n > self.fracao.d {
            self.num_improprio = self.num_improprio + self.fracao.n / self.fracao.d;
            self.fracao.n = self.fracao.n % self.fracao.d;
        }

        self.num_improprio = self.num_improprio + other.num_improprio;
    }

    pub fn sub(&mut self, other: &FractionMixed) {
        self.fracao.sub(&other.fracao);

        self.num_improprio = self.num_improprio - other.num_improprio;
    }

    pub fn mul(&mut self, other: &FractionMixed) {
        let mut f1 = self.get_fraction();
        let f2 = other.get_fraction();
        f1.mul(&f2);
        f1.simplify();

        let fm = f1.get_fraction_mixed();
        self.num_improprio = fm.num_improprio;
        self.fracao.n = fm.fracao.n;
        self.fracao.d = fm.fracao.d;
    }

    pub fn div(&mut self, other: &FractionMixed) {
        let mut f1 = self.get_fraction();
        let f2 = other.get_fraction();

        f1.div(&f2);
        f1.simplify();
        let fm = f1.get_fraction_mixed();

        self.num_improprio = fm.num_improprio;
        self.fracao.n = fm.fracao.n;
        self.fracao.d = fm.fracao.d;
    }
}

#[derive(PartialEq, Debug)]
pub struct Fraction {
    pub n: i32,
    pub d: i32,
}

impl Fraction {
    pub fn simplify(&mut self) {
        let mut p = 2;
        loop {
            if (is_prime(self.n) && is_prime(self.d))
                || (self.n == 1 || self.d == 1) 
                || (p > self.n || p > self.d) {
                break;
            }

            if self.n % p == 0 && self.d % p == 0 {
                self.n = self.n / p;
                self.d = self.d / p;
            } else {
                p = next_prime(p);
            }
        }
    }

    pub fn sum(&mut self, other: &Fraction) {
        if self.d != other.d {
            let lcm = lcm(&mut [self.d, other.d]);
            self.n = lcm / self.d * self.n;
            self.n = self.n + (lcm / other.d * other.n);
            self.d = lcm;
        } else {
            self.n = self.n + other.n;
        }
    }

    pub fn sub(&mut self, other: &Fraction) {
        if self.d != other.d {
            let lcm = lcm(&mut [self.d, other.d]);
            self.n = lcm / self.d * self.n;
            self.n = self.n - (lcm / other.d * other.n);
            self.d = lcm;
        } else {
            self.n = if self.n > other.n {self.n - other.n} else {other.n - self.n};
        }
    }

    pub fn mul_int(&mut self, m: i32) {
        self.n = self.n * m;
    }

    pub fn mul(&mut self, other: &Fraction) {
        self.n = self.n * other.n;
        self.d = self.d * other.d;
    }

    pub fn div(&mut self, other: &Fraction) {
        self.n = self.n * other.d;
        self.d = self.d * other.n;
        self.simplify();
    }

    pub fn apply_lcm(&mut self, mmc: i32) {
        self.n = mmc / self.d * self.n;
        self.d = mmc;
    }

    pub fn get_fraction_mixed(&self) -> FractionMixed {
        FractionMixed { num_improprio: (self.n / self.d), fracao: Fraction {n: self.n % self.d, d: self.d} }
    }

    pub fn get_percentage(&self) -> f64 {
        if self.d == 0 {
            return 0.0;
        }

        self.n as f64 / self.d as f64 * 100.0
    }
}

pub fn lcm(l: &mut [i32]) -> i32 {
    let mut prime = 2;
    let mut lcm = 1; // Least Commom Multiple is equal to MMC.
    let mut ind = 0; 
    let mut all_one = true;
    let mut is_decomposed = false;

    loop {
        while ind < l.len() {
            if l[ind] % prime == 0 {
                l[ind] = l[ind] / prime;
                is_decomposed = true;
            }

            if l[ind] > 1 {
                all_one = false;
            }

            ind += 1;
        }

        if is_decomposed {
            lcm = lcm * prime;
        } else {
            prime = next_prime(prime);
        }

        if all_one {
            break;
        }

        all_one = true;
        is_decomposed = false;
        ind = 0;
    }
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_test1() {
        let mut x = [5, 2];
        assert_eq!(10, lcm(&mut x));
    }

    #[test]
    fn lcm_test2() {
        let mut x = [4, 5, 9, 12, 15];
        assert_eq!(180, lcm(&mut x));
    }

    #[test]
    fn apply_lcm_test1() {
        let mut f = Fraction {n: 10, d: 15};
        f.apply_lcm(180);
        assert_eq!(f, Fraction {n: 120, d: 180});
    }

    #[test]
    fn simplify_test1() {
        let mut f = Fraction {n: 10, d: 15};
        f.simplify();
        assert_eq!(f, Fraction {n: 2, d: 3});
    }

    #[test]
    fn simplify_test2() {
        let mut f = Fraction {n: 3, d: 18};
        f.simplify();
        assert_eq!(f, Fraction {n: 1, d: 6});
    }

    #[test]
    fn fraction_sum_test1() {
        let mut f1 = Fraction {n: 3, d: 15};
        let f2 = Fraction {n: 7, d: 15};
        f1.sum(&f2);
        assert_eq!(f1, Fraction {n: 10, d: 15});
    }

    #[test]
    fn fraction_sum_test2_d_diff() {
        let mut f1 = Fraction {n: 5, d: 6};
        let f2 = Fraction {n: 1, d: 4};
        f1.sum(&f2);
        assert_eq!(f1, Fraction {n: 13, d: 12});
    }

    #[test]
    fn fraction_sum_test3_d_diff() {
        let mut f1 = Fraction {n: 9, d: 10};
        let f2 = Fraction {n: 1, d: 6};
        f1.sum(&f2);
        assert_eq!(f1, Fraction {n: 32, d: 30});
    }

    #[test]
    fn fraction_sum_test4_d_diff() {
        let mut f1 = Fraction {n: 1, d: 2};
        let f2 = Fraction {n: 11, d: 12};
        f1.sum(&f2);
        assert_eq!(f1, Fraction {n: 17, d: 12});
    }

    #[test]
    fn fraction_sum_test5_d_diff() {
        let mut f1 = Fraction {n: 3, d: 4};
        let f2 = Fraction {n: 1, d: 5};
        f1.sum(&f2);
        assert_eq!(f1, Fraction {n: 19, d: 20});
    }

    #[test]
    fn fraction_sub_test1() {
        let mut f1 = Fraction {n: 8, d: 18};
        let f2 = Fraction {n: 5, d: 18};
        f1.sub(&f2);
        assert_eq!(f1, Fraction {n: 3, d: 18});
    }

    #[test]
    fn fraction_sub_test2_d_diff() {
        let mut f1 = Fraction {n: 3, d: 4};
        let f2 = Fraction {n: 5, d: 8};
        f1.sub(&f2);
        assert_eq!(f1, Fraction {n: 1, d: 8});
    }

    #[test]
    fn fraction_sub_test3_d_diff() {
        let mut f1 = Fraction {n: 4, d: 3};
        let f2 = Fraction {n: 1, d: 5};
        f1.sub(&f2);
        assert_eq!(f1, Fraction {n: 17, d: 15});
    }

    #[test]
    fn fraction_sub_test4_d_diff() {
        let mut f1 = Fraction {n: 7, d: 10};
        let f2 = Fraction {n: 5, d: 8};
        f1.sub(&f2);
        assert_eq!(f1, Fraction {n: 3, d: 40});
    }

    #[test]
    fn fraction_sum_sub_test1() {
        let mut f1 = Fraction {n: 1, d: 4};
        let f2 = Fraction {n: 3, d: 5};
        let f3 = Fraction {n: 3, d: 10};
        f1.sum(&f2);
        f1.sub(&f3);
        assert_eq!(f1, Fraction {n: 11, d: 20});
    }

    #[test]
    fn fraction_sub_sum_test1() {
        let mut f1 = Fraction {n: 4, d: 9};
        let f2 = Fraction {n: 1, d: 6};
        let f3 = Fraction {n: 1, d: 3};
        f1.sub(&f2);
        f1.sum(&f3);
        assert_eq!(f1, Fraction {n: 11, d: 18});
    }

    #[test]
    fn fraction_mul_int_test1() {
        let mut f1 = Fraction {n: 2, d: 5};
        f1.mul_int(3);
        assert_eq!(f1, Fraction {n: 6, d: 5});
    }

    #[test]
    fn fraction_mul_test1() {
        let mut f1 = Fraction {n: 1, d: 2};
        let f2 = Fraction {n: 1, d: 4};
        f1.mul(&f2);
        assert_eq!(f1, Fraction {n: 1, d: 8});
    }

    #[test]
    fn fraction_div_test1() {
        let mut f1 = Fraction {n: 8, d: 3};
        let f2 = Fraction {n: 1, d: 3};
        
        f1.div(&f2);

        assert_eq!(f1, Fraction {n: 8, d: 1});
    }

    #[test]
    fn fraction_div_test2() {
        let mut f1 = Fraction {n: 8, d: 3};
        let f2 = Fraction {n: 2, d: 3};
        
        f1.div(&f2);

        assert_eq!(f1, Fraction {n: 4, d: 1});
    }

    #[test]
    fn fraction_div_test3() {
        let mut f1 = Fraction {n: 8, d: 3};
        let f2 = Fraction {n: 3, d: 3};
        
        f1.div(&f2);

        assert_eq!(f1, Fraction {n: 8, d: 3});
    }

    #[test]
    fn fraction_div_test4() {
        let mut f1 = Fraction {n: 2, d: 5};
        let f2 = Fraction {n: 7, d: 3};
        
        f1.div(&f2);

        assert_eq!(f1, Fraction {n: 6, d: 35});
    }

    #[test]
    fn fraction_div_test5() {
        let mut f1 = Fraction {n: 3, d: 5};
        let f2 = Fraction {n: 1, d: 2};
        
        f1.div(&f2);
        let fm = f1.get_fraction_mixed();
        
        assert_eq!(f1, Fraction {n: 6, d: 5});
        assert_eq!(fm, FractionMixed {num_improprio: 1, fracao: Fraction { n: 1, d: 5 }})
    }

    #[test]
    fn get_fraction_test1() {
        let fm = FractionMixed {num_improprio: 5, fracao: Fraction { n: 1, d: 4 }};
        let f = fm.get_fraction();
        assert_eq!(f, Fraction {n: 21, d: 4});
    }

    #[test]
    fn get_fraction_mixed_test1() {
        let f = Fraction {n: 21, d: 4};
        let fm = f.get_fraction_mixed();
        assert_eq!(fm, FractionMixed {num_improprio: 5, fracao: Fraction { n: 1, d: 4 }})
    }

    #[test]
    fn get_fraction_mixed_test2() {
        let f = Fraction {n: 7, d: 4};
        let fm = f.get_fraction_mixed();
        assert_eq!(fm, FractionMixed {num_improprio: 1, fracao: Fraction { n: 3, d: 4 }})
    }

    #[test]
    fn fraction_mixed_sum_test1() {
        let mut fm1 = FractionMixed {num_improprio: 3, fracao: Fraction { n: 3, d: 5 }};
        let fm2 = FractionMixed {num_improprio: 5, fracao: Fraction { n: 4, d: 5 }};
        fm1.sum(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 9, fracao: Fraction { n: 2, d: 5 }})
    }

    #[test]
    fn fraction_mixed_sum_test2() {
        let mut fm1 = FractionMixed {num_improprio: 2, fracao: Fraction { n: 4, d: 7 }};
        let fm2 = FractionMixed {num_improprio: 3, fracao: Fraction { n: 2, d: 7 }};
        fm1.sum(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 5, fracao: Fraction { n: 6, d: 7 }})
    }

    #[test]
    fn fraction_mixed_sum_test3() {
        let mut fm1 = FractionMixed {num_improprio: 19, fracao: Fraction { n: 3, d: 18 }};
        let fm2 = FractionMixed {num_improprio: 18, fracao: Fraction { n: 2, d: 3 }};
        fm1.sum(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 37, fracao: Fraction { n: 15, d: 18 }})
    }

    #[test]
    fn fraction_mixed_sub_test1() {
        let mut fm1 = FractionMixed {num_improprio: 2, fracao: Fraction { n: 5, d: 8 }};
        let fm2 = FractionMixed {num_improprio: 1, fracao: Fraction { n: 2, d: 8 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 1, fracao: Fraction { n: 3, d: 8 }})
    }

    /*
    #[test]
    fn fraction_mixed_sub_test2() {
        let mut fm1 = FractionMixed {num_improprio: 3, fracao: Fraction { n: 2, d: 5 }};
        let fm2 = FractionMixed {num_improprio: 2, fracao: Fraction { n: 3, d: 3 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 0, fracao: Fraction { n: 4, d: 5 }})
    }

    #[test]
    fn fraction_mixed_sub_test3() {
        let mut fm1 = FractionMixed {num_improprio: 7, fracao: Fraction { n: 1, d: 6 }};
        let fm2 = FractionMixed {num_improprio: 4, fracao: Fraction { n: 5, d: 6 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 2, fracao: Fraction { n: 2, d: 6 }})
    }
    */

    #[test]
    fn fraction_mixed_sub_test4() {
        let mut fm1 = FractionMixed {num_improprio: 9, fracao: Fraction { n: 2, d: 8 }};
        let fm2 = FractionMixed {num_improprio: 6, fracao: Fraction { n: 1, d: 8 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 3, fracao: Fraction { n: 1, d: 8 }})
    }

    #[test]
    fn fraction_mixed_sub_test5() {
        let mut fm1 = FractionMixed {num_improprio: 7, fracao: Fraction { n: 6, d: 9 }};
        let fm2 = FractionMixed {num_improprio: 3, fracao: Fraction { n: 2, d: 5 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 4, fracao: Fraction { n: 12, d: 45 }})
    }

    #[test]
    fn fraction_mixed_mul_test1() {
        let mut fm1 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 3, d: 4}};
        let fm2 = FractionMixed {num_improprio: 7, fracao: Fraction {n: 1, d: 5}};
        
        fm1.mul(&fm2);
        
        assert_eq!(fm1, FractionMixed {num_improprio: 12, fracao: Fraction {n: 3, d: 5}})
    }

    #[test]
    fn fraction_mixed_mul_test2() {
        let mut fm1 = FractionMixed {num_improprio: 3, fracao: Fraction {n: 3, d: 7}};
        let fm2 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 2, d: 1}};
        
        fm1.mul(&fm2);
        
        let f1 = fm1.get_fraction();

        assert_eq!(fm1, FractionMixed {num_improprio: 6, fracao: Fraction {n: 6, d: 7}});
        assert_eq!(f1, Fraction {n: 48, d: 7});
    }

    #[test]
    fn fraction_mixed_mul_test3() {
        let mut fm1 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 1, d: 3}};
        let fm2 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 6, d: 7}};
        
        fm1.mul(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 1, fracao: Fraction {n: 1, d: 7}});
        assert_eq!(f1, Fraction {n: 8, d: 7});
    }

    #[test]
    fn fraction_mixed_mul_test4() {
        let mut fm1 = FractionMixed {num_improprio: 3, fracao: Fraction {n: 1, d: 2}};
        let fm2 = FractionMixed {num_improprio: 3, fracao: Fraction {n: 1, d: 2}};
        
        fm1.mul(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 12, fracao: Fraction {n: 1, d: 4}});
        assert_eq!(f1, Fraction {n: 49, d: 4});
    }

    #[test]
    fn fraction_mixed_mul_test5() {
        let mut fm1 = FractionMixed {num_improprio: 4, fracao: Fraction {n: 1, d: 1}};
        let fm2 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 3, d: 4}};
        
        fm1.mul(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 8, fracao: Fraction {n: 3, d: 4}});
        assert_eq!(f1, Fraction {n: 35, d: 4});
    }

    #[test]
    fn fraction_mixed_div_test1() {
        let mut fm1 = FractionMixed {num_improprio: 3, fracao: Fraction {n: 3, d: 8}};
        let fm2 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 9, d: 1}};
        
        fm1.div(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 0, fracao: Fraction {n: 3, d: 8}});
        assert_eq!(f1, Fraction {n: 3, d: 8});
    }

    #[test]
    fn fraction_mixed_div_test2() {
        let mut fm1 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 8, d: 1}};
        let fm2 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 4, d: 5}};
        
        fm1.div(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 4, fracao: Fraction {n: 4, d: 9}});
        assert_eq!(f1, Fraction {n: 40, d: 9});
    }

    #[test]
    fn fraction_mixed_div_test3() {
        let mut fm1 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 5, d: 8}};
        let fm2 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 1, d: 3}};
        
        fm1.div(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 0, fracao: Fraction {n: 15, d: 32}});
        assert_eq!(f1, Fraction {n: 15, d: 32});
    }

    #[test]
    fn fraction_mixed_div_test4() {
        let mut fm1 = FractionMixed {num_improprio: 1, fracao: Fraction {n: 7, d: 9}};
        let fm2 = FractionMixed {num_improprio: 0, fracao: Fraction {n: 4, d: 5}};
        
        fm1.div(&fm2);
        
        let f1 = fm1.get_fraction();
        
        assert_eq!(fm1, FractionMixed {num_improprio: 2, fracao: Fraction {n: 2, d: 9}});
        assert_eq!(f1, Fraction {n: 20, d: 9});
    }
}

#[cfg(test)]
mod fraction {
    use super::*;

    #[test]
    fn get_percentage_test1() {
        assert_eq!((Fraction {n: 6, d: 20}).get_percentage(), 30f64);
    }

    #[test]
    fn get_percentage_test2() {
        assert_eq!((Fraction {n: 7, d: 10}).get_percentage(), 70f64);
    }

    #[test]
    fn get_percentage_test3() {
        assert_eq!((Fraction {n: 9, d: 5}).get_percentage(), 180f64);
    }

    #[test]
    fn get_percentage_test4() {
        assert_eq!((Fraction {n: 9, d: 0}).get_percentage(), 0f64);
    }

    #[test]
    fn get_percentage_test5() {
        assert_eq!((Fraction {n: 9, d: -5}).get_percentage(), -180f64);
    }

    #[test]
    fn get_percentage_test6() {
        assert_eq!((Fraction {n: 1, d: 3}).get_percentage(), 1.0/3.0*100.0);
    }

    #[test]
    fn get_percentage_test7() {
        assert_eq!((Fraction {n: 1, d: -3}).get_percentage(), 1.0/-3.0*100.0);
    }

    #[test]
    fn get_percentage_test8() {
        assert_eq!((Fraction {n: 12, d: 25}).get_percentage(), 48.0);
    }
}