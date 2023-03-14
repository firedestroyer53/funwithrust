// fn sumofnumbersdivisiblebythreeorfive(){
//     let mut sum = 0;
//         for n in 1..1000{
//             if n % 5 == 0{
//                 sum += n;
//             }
//             else if n % 3 == 0{
//                 sum += n;
//             }
//         }
//         println!("{}", sum);
// }

// fn sumofevenfibonaccinumbers(){
//     let mut sum = 0;
//     let (mut a, mut b) = (1, 2);
//     while b <= 4_000_000 {
//         if b % 2 == 0 {
//             sum += b;
//         }
//         let c = a + b;
//         a = b;
//         b = c;
//     }
//     println!("{}", sum); // prints 4613732
// }

// fn largestprimefactor(){
//     let mut num = 600_851_475_143;
//     let mut answer = 0;

//     // Check if the number is divisible by 2
//     while num % 2 == 0 {
//         answer = 2;
//         num /= 2;
//     }

//     // Check if the number is divisible by odd numbers
//     let mut i = 3;
//     while i <= (num as f64).sqrt() as i64 {
//         if num % i == 0 {
//             answer = i;
//             num /= i;
//         } else {
//             i += 2; // Skip even numbers
//         }
//     }

//     // If there is a factor greater than the square root of the number,
//     // it must be a prime factor
//     if num > 2 {
//         answer = num;
//     }

//     println!("{}", answer);

// }

// fn is_palindrome(x: i32) -> bool {
//     let mut temp = x;
//     let mut reverse = 0;
//     while temp > 0 {
//         reverse = reverse * 10 + temp % 10;
//         temp /= 10;
//     }
//     x == reverse
// }


// fn sieve(primes: &mut Vec<i32>, factor: i32) {
//     let mut i = 0;
//     while i < primes.len() {
//         let value = primes[i];
//         if value != factor {
//             if value % factor == 0 {
//                 primes.remove(i);
//             }
//         }
//         i += 1;
//     }
// }


// fn Largestproductinaseries() {
//     //
//     let mut start;
//     let mut end;
//     let mut biggest = 0;
//     let number = "the number (placeholder bc its long af)";
//     let array = arraymaker(number);
//     for i in 0..=987{
//         start = i;
//         end = i+13;
//         let mut product: i64 = 1;
//         for j in start..end{
//             product *= array[j as usize] as i64;
//         }
//         if product > biggest{
//             biggest = product;
//         }
        
//     }
//     println!("{}", biggest);
// }

// fn arraymaker(i: &str) -> Vec<u32>{
//     let digits: Vec<u32> = i
//         .chars()
//         .map(|c| c.to_digit(10).unwrap())
//         .collect();
//     return digits;
// }

// fn is_triplet(a:i32,b:i32,c:i32) -> bool{
//     a.pow(2) + b.pow(2) == c.pow(2)
// }

// fn primesum(){
//     let mut primes = bitvec![0; 2_000_000];
//     let mut sum = 0;

//     // 0 and 1 are not prime
//     primes.set(0, true);
//     primes.set(1, true);

//     for i in 2..2_000_000 {
//         if !primes[i] {
//             sum += i;
//             let mut j = i * i;
//             while j < 2_000_000 {
//                 primes.set(j, true);
//                 j += i;
//             }
//         }
//     }

//     println!("{}", sum);

// }