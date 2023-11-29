use std::char;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //    unimplemented!("Is the Luhn checksum for {code} valid?");
    let mut s : Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    if s.is_empty() || s.len() <=1 {
        return false;
    }
    for c in &s {
        if ! c.is_digit(10) {
            return false;
        }
    }

    let mut skip : bool = true;
    let mut sum : u32 = 0;
    //for i in (s.len()-1..=0).rev() {
    for c in s.iter_mut().rev() {
        let mut val : u32 = c.to_digit(10).unwrap();
        println!("val = {val}");
       if !skip {
           val *= 2;
           if val > 9 {
               val -= 9;
           }
           //s[i] = char::from_digit(val, 10).unwrap();
           *c = char::from_digit(val, 10).unwrap();
           skip = true;
       } else {
           skip = false;
       }
       sum += val;
    }
    println!("s = {:?} sum = {sum}" , s);
    if sum % 10 == 0 {
        return true;
    }
    return false;
}
