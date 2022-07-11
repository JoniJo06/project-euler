use crate::math;
use std::ops::Index;

pub fn multiples_of_3_or_5() {
    println!("0001# multiples of 3 or 5");

    let mut result = 0;
    let mut numbers: Vec<i32> = Vec::new();

    for n in 0..1000 {
        if n % 3 == 0 {
            numbers.push(n);
            continue;
        }
        if n % 5 == 0 {
            numbers.push(n);
            continue;
        }
    }

    for number in numbers.iter() {
        result += number;
    }

    println!("Result: {}", result);
}

pub fn even_fibonacci_numbers() {
    println!("0002# even fibonacci numbers");

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

pub fn largest_prime_factor() {
    println!("0003# largest prime factor");

    let mut root_number = 600851475143_f64;

    let mut result = 0_f64;

    while root_number % 2.0 == 0.0 {
        result = 2.0;
        root_number /= 1.0;
    }

    for i in (3..(root_number.sqrt() + 1.0) as u64).step_by(2) {
        while root_number % i as f64 == 0.0 {
            result = i as f64;
            root_number /= i as f64;
        }
    }

    if root_number > 2.0 {
        result = root_number;
    }

    println!("Result: {}", result);
}

pub fn largest_palindrome_product() {
    println!("0004# largest palindrome product");
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

pub fn smallest_multiple() {
    println!("0005# smallest multiple");
    let result;
    let mut n: u128 = 20;
    loop {
        let mut dividable: bool = true;
        for i in 1..20 {
            if n % i != 0 {
                dividable = false;
            }
        }
        if dividable {
            result = n;
            break;
        }

        n += 20;
    }

    println!("Result: {}", result);
}

pub fn sum_square_difference() {
    println!("0006# sum square difference");

    let n = 100;
    let mut sum_square = 0;

    for i in 1..n + 1 {
        sum_square += i32::pow(i, 2);
    }

    let mut square_sum = 0;
    for i in 1..n + 1 {
        square_sum += i;
    }
    square_sum = i32::pow(square_sum, 2);

    let result = square_sum - sum_square;
    println!("Result: {}", result);
}

pub fn _10001st_prime() {
    println!("0007# 10001st prime");

    let mut n = 10001;
    let mut i = 2;
    while n > 0 {
        if math::is_prime(i) {
            n -= 1;
        }

        i += 1;
    }
    i -= 1;

    println!("Result: {}", i);
}


pub fn largest_product_in_a_series() {
    println!("0008# largest product in a series");

    let mut result = 0;

    let input = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";

    type MyType = u64;

    let numbers: String = input.split('\n').collect();
    let numbers: Vec<MyType> = numbers
        .chars()
        .map(|x| x.to_string().parse::<MyType>().unwrap())
        .collect();

    let n = 13;

    for i in 0..numbers.len() - n {
        let mut product = 1;
        for j in 0..n {
            product *= numbers.index(i + j);
        }
        if product > result {
            result = product;
        }
    }

    println!("Result: {}", result);
}

pub fn special_pythagorean_triplet() {
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

                if a.pow(2) + b.pow(2) != c.pow(2) {
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

pub fn summation_of_primes() {
    println!("0010# summation of primes");
    let mut result: u64 = 0;

    for num in 0..2_000_000_u64 {
        if math::is_prime(num) {
            result += num;
        }
    }

    println!("Result: {}", result);
}