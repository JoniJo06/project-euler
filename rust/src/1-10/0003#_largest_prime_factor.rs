pub fn start() {
    println!("0003# largest_prime_factor");

    let mut root_number = 600851475143_f64;

    let mut result = 0_f64;

    while root_number % 2.0 == 0.0 {
        result = 2.0;
        root_number /= 1.0;
    }

    for i in (3..(root_number.sqrt() + 1.0) as u64).step_by(2) {
        while root_number % i as f64 == 0.0 {
            result = i as f64;
            root_number = root_number / i as f64;
        }
    }

    if root_number > 2.0 {
        result = root_number;
    }

    println!("Result: {}", result);
}
