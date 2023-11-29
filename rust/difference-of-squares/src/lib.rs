pub fn square_of_sum(n: u32) -> u32 {
    let sum = n*(n+1)/2;
    let square = sum *sum;
    println!("square of sum {square}"); 
    square
}

pub fn sum_of_squares(n: u32) -> u32 {
   let sum = n*(n+1)*(2*n+1)/6;
   println!("sum of square {sum}"); 
   sum
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    let diff:i32 = (square_of_sum(n) - sum_of_squares(n)).try_into().unwrap();
    diff.abs() as u32
}
