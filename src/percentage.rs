use crate::fraction::Fraction;

pub fn get_fraction(x: f64) -> Fraction {
    let mut perc = x;
    let mut perc_trunc = perc.trunc();
    let mut i_pow = 0;

    while (perc - perc_trunc <= 0.0) == false {
        perc = (perc * 10.0 * 1000.0).round() / 1000.0; // because 0.14 * 10 is 1.4000000000000001, and it bug the calc -_-
        perc_trunc = perc.trunc();
        i_pow = i_pow + 1;
    }

    let mut f = Fraction {n: perc as i32, d: 100};
    if i_pow > 1 {
        println!("{}", i_pow);
        f.d = 10_i32.pow(i_pow);
    }

    //f.simplify();
    f
}

#[cfg(test)]
mod get_fraction {
    use crate::{percentage::get_fraction, fraction::Fraction};


    #[test]
    fn test1() {
        assert_eq!(get_fraction(15.0), Fraction {n: 15, d: 100});
    }

    #[test]
    fn test2() {
        assert_eq!(get_fraction(0.08), Fraction {n: 8, d: 100});
    }

    #[test]
    fn test3() {
        assert_eq!(get_fraction(0.085), Fraction {n: 85, d: 1000});
    }

    #[test]
    fn test4() {
        assert_eq!(get_fraction(0.05), Fraction {n: 5, d: 100});
    }

    #[test]
    fn test5() {
        assert_eq!(get_fraction(0.14), Fraction {n: 14, d: 100});
    }

    // panic!!  |
    //         \ /
    #[test]
    fn test6() {
        assert_eq!(get_fraction(0.3), Fraction {n: 14, d: 100});
    }

    #[test]
    fn test7() {
        assert_eq!(get_fraction(33.0), Fraction {n: 14, d: 100});
    }
    //         / \
    // panic!!  |

}