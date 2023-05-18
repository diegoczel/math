use crate::prime::{is_prime, next_prime};

#[derive(Debug)]
pub struct FractionMixed {
    pub num_improprio: i32,
    pub fracao: Fraction,
}

impl FractionMixed {
    pub fn get_fraction(&self) -> Fraction {
        Fraction {n: (self.fracao.d * self.num_improprio) + self.fracao.n, d: self.fracao.d}
    }
}

#[derive(Debug)]
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
        if self.d == other.d {
            self.n = self.n + other.n;
        }
    }

    pub fn sub(&mut self, other: &Fraction) {
        if self.d == other.d {
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