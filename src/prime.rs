pub fn next_prime(p: i32) -> i32 {
    if p < 2 {
        return 2;
    }
    let mut x = if p % 2 == 0 { p + 1 } else { p + 2 };
    loop {
        if is_prime(x) {
            break;
        }
        x += 2;
    }
    x
}

pub fn is_prime(x: i32) -> bool {
    if x % 2 == 0 && x > 2 {
        return false;
    }
    if x == 1 {
        return false;
    }

    for i in (3..x).step_by(2) {
        if x % i == 0 {
            return false;
        }
    }

    true
}

pub fn divs_of_num(num: i32) -> Vec<i32> {
    let mut v = vec![0; 0];

    v.push(1);

    for x in 2..num{
        if num % x == 0 {
            v.push(x);
        }
    }

    v.push(num);

    v
}

#[cfg(test)]
mod tests {
    use crate::prime::divs_of_num;

    #[test]
    fn divs_of_num_test1() {
        assert_eq!(divs_of_num(120), vec![1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60, 120]);
        //assert_eq!(divs_of_num(120), vec![1, 120, 2, 60, 3, 40, 4, 30, 5, 24, 6, 20, 8, 15, 10, 12]);
    }

    #[test]
    fn divs_of_num_test2() {
        assert_eq!(divs_of_num(48), vec![1, 2, 3, 4, 6, 8, 12, 16, 24, 48]);
        //assert_eq!(divs_of_num(48), vec![1, 48, 2, 24, 3, 16, 4, 12, 6, 8]);
    }

    #[test]
    fn divs_of_num_test3() {
        assert_eq!(divs_of_num(56), vec![1, 2, 4, 7, 8, 14, 28, 56]);
        //assert_eq!(divs_of_num(48), vec![1, 48, 2, 24, 3, 16, 4, 12, 6, 8]);
    }
}