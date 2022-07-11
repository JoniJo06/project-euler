pub fn start() {
    println!("0009# special pythagorean triplet");
    let result;

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;

    'c: loop {
        c += 1;

        'b: loop {
            b += 1;

            'a: loop {
                a += 1;
                if a >= b {
                    a = 1;
                    continue 'b;
                }
                if b >= c {
                    b = 1;
                    continue 'c;
                }

                if !(a.pow(2) + b.pow(2) == c.pow(2)) {
                    continue 'a;
                }

                if a + b + c == 1000 {
                    result = a * b * c;
                    break 'c;
                }
            }
        }
    }

    println!("Result: {}", result);
}
