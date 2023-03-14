fn main(){
    //first triangle number with over 500 divisors
    let mut i = 1;
    let mut triangle = 0;
    while factors(triangle).len() < 500 {
        triangle += i;
        i += 1;
    }
    println!("{}", triangle);
}

//return factors of a number
fn factors(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
}