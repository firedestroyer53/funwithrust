use rand::prelude::*;

fn main(){
    let mut rng = rand::thread_rng();
    let x:i64 = rng.gen_range(1..10000000000);
    println!("{}", x);
    println!("{}", evenorodd(x));
}

fn evenorodd(num: i64) -> String{
    if num % 2 == 0{
        return "Even".to_string();
    }
    else{
        return "Odd".to_string();
    }
}