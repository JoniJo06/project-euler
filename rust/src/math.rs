use std::borrow::Borrow;

pub fn is_prime(k: u64) -> bool {
  if k <= 1 {
    return false;
  }

  if k == 2 || k == 3 {
    return true;
  }

  if k % 2 == 0 || k % 3 == 0 {
    return false;
  }

  let mut i = 5;
  loop {
    if i * i > k {
      break;
    }

    if k % i == 0 || k % (i + 2) == 0 {
      return false;
    }

    i += 6;
  }

  true
}

pub fn number_to_english(number: usize) -> String {
	let ones: Vec<&str> = vec![
		"zero",
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
		"ten",
		"eleven",
		"twelve",
		"thirteen",
		"fourteen",
		"fifteen",
		"sixteen",
		"seventeen",
		"eighteen",
		"nineteen",
	];

  
	let tens: Vec<&str> = vec![
    "twenty", 
    "thirty", 
    "forty", 
    "fifty", 
    "sixty", 
    "seventy", 
    "eighty", 
    "ninety"
    ];

	if number <= 19 {
    return ones[number].to_string();
  } 

  // let a: usize = 10;
  // let b: usize = 3;

  // println!("{}", a / b);

  
	if number >= 20 && number <= 90 && number % 10 == 0 {
    return tens[number / 10 - 2].to_string();
  }
  
	if number >= 21 && number <= 99 {
    return format!("{}-{}",tens[number / 10 - 2], ones[number % 10]);
  }
  
	if number >= 100 && number <= 900 && number % 100 == 0 {
    return format!("{} hundred",ones[number / 100]);
  } 
  
	if number >= 101 && number <= 999 {
    return format!("{} hundred and {}", ones[number / 100], number_to_english(number % 100));
  } 
  
	if number == 1000 {
    return "one thousand".to_string();

  }

  panic!("Invalid number: {}", number);
}
