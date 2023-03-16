fn main() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(vec![0, 1, 2, 3]), vec![6, 0, 0, 0]);
    assert_eq!(product_except_self(vec![-1, -2, -3, -4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(vec![1, 2]), vec![2, 1]);
}


