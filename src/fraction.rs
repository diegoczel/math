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
        self.fracao.n = self.fracao.n + other.fracao.n;
        
        if self.fracao.n > self.fracao.d {
            self.num_improprio = self.num_improprio + self.fracao.n / self.fracao.d;
            self.fracao.n = self.fracao.n % self.fracao.d;
        }

        self.num_improprio = self.num_improprio + other.num_improprio;
    }

    pub fn sub(&mut self, other: &FractionMixed) {
        if self.fracao.n < other.fracao.n {
            self.num_improprio = self.num_improprio - 1;
            self.fracao.n = self.fracao.n + self.fracao.d;
        }

        self.fracao.n = self.fracao.n - other.fracao.n;
        self.num_improprio = self.num_improprio - other.num_improprio;
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

    pub fn apply_lcm(&mut self, mmc: i32) {
        self.n = mmc / self.d * self.n;
        self.d = mmc;
    }

    pub fn get_fraction_mixed(&self) -> FractionMixed {
        FractionMixed { num_improprio: (self.n / self.d), fracao: Fraction {n: self.n % self.d, d: self.d} }
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
    fn fraction_mixed_sub_test1() {
        let mut fm1 = FractionMixed {num_improprio: 2, fracao: Fraction { n: 5, d: 8 }};
        let fm2 = FractionMixed {num_improprio: 1, fracao: Fraction { n: 2, d: 8 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 1, fracao: Fraction { n: 3, d: 8 }})
    }

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

    #[test]
    fn fraction_mixed_sub_test4() {
        let mut fm1 = FractionMixed {num_improprio: 9, fracao: Fraction { n: 2, d: 8 }};
        let fm2 = FractionMixed {num_improprio: 6, fracao: Fraction { n: 1, d: 8 }};
        fm1.sub(&fm2);
        assert_eq!(fm1, FractionMixed {num_improprio: 3, fracao: Fraction { n: 1, d: 8 }})
    }
}