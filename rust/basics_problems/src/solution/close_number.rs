use std::i32;

#[allow(unused)]
pub fn one_close_number_fn(n: i32, m: i32) -> i32 {
    let mut closest: i32 = 0;
    let mut min_value: i32 = i32::MAX;

    let mut i = n - i32::abs(m);

    loop {
        let mut difference: i32 = i32::abs(n - i);

        if i % m == 0 {
            if difference < min_value
                || (difference == min_value && i32::abs(i) > i32::abs(closest))
            {
                closest = i;
                min_value = difference;
            }
        }

        if i >= n + i32::abs(m) {
            break closest;
        }

        i += 1;
    }
}

#[allow(unused)]
pub fn two_close_number_fn(n: i32, m: i32) -> i32 {
    // calculate q = n / m
    let q = n / m;

    // calculate n1 = m * q
    let n1 = m * q;

    // calculate n2 = m * (q + 1) or m * (q - 1)
    let mut n2 = if n * m > 0 {
        m * (q + 1)
    } else {
        m * (q - 1)
    };

    // return minimum value (n1 and n2) of difference from n
    if i32::abs(n1 - n) < i32::abs(n2 - n) {
        n1
    } else {
        n2
    }
}
