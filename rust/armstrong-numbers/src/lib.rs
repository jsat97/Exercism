pub fn is_armstrong_number(num: u32) -> bool {
    // calculate the # of digits
    let mut no_of_digits = 0;
    let mut i = num;
    while i != 0 {
        i /= 10;
        no_of_digits += 1;
    }
    // armstrong ? sum of (digit ^{total # of digits)) == number
    let mut sum = 0u64;
    let mut i = num;
    while i != 0 {
        let digit = i % 10;
        sum += digit.pow(no_of_digits) as u64;
        i /= 10;
    }
    if sum > u32::MAX.into() {
        return false;
    }
    num == sum as u32
}
