

fn main(){
    println!("Hello World");
}

fn evenorodd(num: i32) -> String{
    if num % 2 == 0{
        return "Even".to_string();
    }
    else{
        return "Odd".to_string();
    }
}