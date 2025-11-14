#[allow(unused)]
pub fn digit_extraction(mut n: i32) -> i32 {
    // init sum variable
    let mut sum: i32 = 0;
    loop {
        // extracting the last digit using n % 10
        let mut last_digit = n % 10;

        // adding it to the sum
        sum += last_digit;

        // removing it by dividing n by 10 using integer division
        n /= 10;

        if n <= 0 {
            break;
        }
    }
    sum
}
#[allow(unused)]
pub fn recursion(n: i32) -> i32 {
    if n != 0 {
        n % 10 + recursion(n / 10)
    } else {
        0
    }
}

#[allow(unused)]
pub fn convert_to_string(n: i32) -> u32 {
    // Convert the number to a string and iterate through each character (digit).
    let str = n.to_string();
    let mut sum: u32 = 0;
    // For each character, subtract the ASCII value of '0' to get the actual digit,
    for i in str.chars() {
        sum += i.to_digit(10).unwrap();
    }
    sum
}
