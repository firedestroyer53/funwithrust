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

// fn sum_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
//     let mut result = Vec::new();
//     for i in 0..arr1.len() {
//         result.push(arr1[i] + arr2[i]);
//     }
//     result
// }


// fn longest_word(text: &str) -> &str {
//     let mut longest = "";
//     let textlist = text.split(" ").collect::<Vec<&str>>();
//     for i in textlist{
//         if i.len() > longest.len(){
//             longest = i;
//         }
//     }
//     longest
// }

//fn find_pairs(vec: &[i32], sum: i32) -> Vec<(i32, i32)> {
//     let mut pairs = Vec::new();
//     for i in 0..vec.len() {
//         for j in i + 1..vec.len() {
//             if vec[i] + vec[j] == sum {
//                 pairs.push((vec[i], vec[j]));
//             }
//         }
//     }
//     pairs
// }

// fn find_triplets(vec: &[i32], sum: i32) -> Vec<(i32, i32, i32)> {
//     let mut triplets = Vec::new();
//     for i in 0..vec.len() {
//         for j in i + 1..vec.len() {
//             for k in j + 1..vec.len() {
//                 if vec[i] + vec[j] + vec[k] == sum {
//                     triplets.push((vec[i], vec[j], vec[k]));
//                 }
//             }
//         }
//     }
//     triplets
// }

// fn find_anagrams(vec: &[&str]) -> Vec<Vec<&str>> {
//     let mut anagrams = Vec::new();
//     for i in 0..vec.len() {
//         for j in i + 1..vec.len() {
//             if is_anagram(vec[i], vec[j]) {
//                 anagrams.push(vec![vec[i], vec[j]]);
//             }
//         }
//     }
//     anagrams
// }

// fn is_anagram(word1: &str, word2: &str) -> bool {
//     let mut sorted1: Vec<char> = word1.to_lowercase().chars().collect();
//     let mut sorted2: Vec<char> = word2.to_lowercase().chars().collect();
//     sorted1.sort();
//     sorted2.sort();
//     sorted1 == sorted2
// }

// fn longest_palindrome(s: &str) -> i32 {
//     let mut count = 0;
//     let mut odd = false;
//     let mut map = std::collections::HashMap::new();
//     for c in s.chars() {
//         let counter = map.entry(c).or_insert(0);
//         *counter += 1;
//     }
//     for (_, v) in map {
//         if v % 2 == 0 {
//             count += v;
//         } else {
//             count += v - 1;
//             odd = true;
//         }
//     }
//     if odd {
//         count + 1
//     } else {
//         count
//     }
// }