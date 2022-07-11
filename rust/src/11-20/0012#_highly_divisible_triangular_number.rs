pub fn start() {
    println!("0012# highly divisible triangular numbers&");

    let n = 500;

    let mut i: u128 = 1;
    let mut number = 0;
    'main: loop {
        number += i;

        println!("###########################");
        println!("Number: {}", number);
        let mut divisible_count = 0;

        for j in 1..(number as f64).sqrt() as u128 + 1 {
            if number % j == 0 {
                divisible_count += 2;
                print!("{} ", j);
            }
        }
        print!("\n");
        println!("Divisible Count: {}", divisible_count);
        if divisible_count > n {
            break 'main;
        }
        i += 1;
    }
}
