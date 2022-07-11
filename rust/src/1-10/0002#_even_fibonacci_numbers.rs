pub fn start() {
    println!("0002# even_fibonacci_numbers");

    let mut result: i32 = 0;

    let mut a = 1;
    let mut b = 2;

    while b < 4_000_000 {
        if b % 2 == 0 {
            result += b;
        }
        let temp = a + b;
        a = b;
        b = temp;
    }

    println!("Result: {}", result);
}
