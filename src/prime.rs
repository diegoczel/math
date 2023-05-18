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