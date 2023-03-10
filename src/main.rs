use std::io;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num = input.trim().parse::<i64>().unwrap();

    if evenorodd(num){
        println!("{} is even", num);
    }
    else{
        println!("{} is odd", num)
    }
}

fn evenorodd(x: i64) -> bool {
    if x%2 == 0 {
        return true;
    }
    else {
        return false;
    }
}