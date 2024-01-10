use rand::Rng;
use std::env;
use std::process::exit;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 || !check_key_length(&args[1]) {
		println!("Usage: {} <product_key / generate>", args[0]);
		println!("Example: win95 XXX-XXXXXXX");
		exit(1);
	}

	let product_key: String = String::from(&args[1]);
	if product_key == "generate" {
		let key = generate_product_key();
		println!(
			"Windows 95 Key: {}-{}",
			format!("{:03}", key[0]),
			format!("{:07}", key[1])
		);
	} else {
		if validate_product_key(parse_input(&product_key)) {
			println!("Valid key: {}", product_key);
		} else {
			println!("Invalid key: {}", product_key);
		}
	}

	fn check_key_length(input: &String) -> bool {
		if input.trim().len() == 11 || input.trim().len() == 8 {
			return true;
		} else {
			false
		}
	}

	fn parse_input(input: &String) -> Vec<usize> {
		let key: Vec<usize>;

		key = vec![
			input[0..3].trim().parse().expect("Not a number!\n"),
			input[4..11].trim().parse().expect("Not a number!\n"),
		];
		key
	}
	fn generate_product_key() -> Vec<usize> {
		loop {
			let first_half = rand::thread_rng().gen_range(000..=998);
			let second_half = rand::thread_rng().gen_range(0_000_000..=8_888_888);
			let key: Vec<usize> = vec![first_half, second_half];

			if validate_product_key(key.clone()) {
				break key;
			}
		}
	}

	fn validate_product_key(key: Vec<usize>) -> bool {
		let first_three = key[0];
		let last_seven = key[1];

		// First half validation
		if (3..=9).contains(&(first_three / 111)) && first_three % 111 == 0 {
			return false;
		}
		// Second half validation
		if second_check(last_seven) {
			return true;
		}
		false
	}

	fn second_check(number: usize) -> bool {
		let mut n = number;
		let mut sum = 0;

		while n > 0 {
			if n % 10 == 9 {
				return false; // If the last digit is 9, the number contains 9
			}
			sum += n % 10;
			n /= 10; // Move to the next digit
		}
		if sum % 7 == 0 {
			return true;
		}
		false
	}
}
