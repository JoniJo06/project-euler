pub fn start() {
    println!("0004#_largest_palindrome_product");
    let mut result = 0;

    for i in 100..999 {
        for j in 100..999 {
            let product = i * j;
            let rev_product = product
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if product == rev_product && product > result {
                result = product;
            }
        }
    }

    println!("Result: {}", result);
}
