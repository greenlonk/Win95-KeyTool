/*Dependencies*/
use std::env;
use rand::Rng;

/*Code*/
fn main() {
	// Get arguments from terminal
	let args: Vec<String> = env::args()
		.collect();
	println!("[Win95 Key Tool]");
	// Check how many arguments are available
	if args
		.len() != 2 {
		eprintln!("Usage: {} <product key> / generate", args[0]);
		std::process::exit(1);
	}
	// Load argument into variable and check if it is called "generate"
	// Everything else will be checked if it is a valid product key
	let product_key: &str = args[1]
		.as_str();
	if product_key == "generate" {
		println!("Windows 95 key: {}", generate_product_key());
	}
	else if validate_product_key(product_key) {
		println!("Valid key: {}", product_key);
	}
	else {
		println!("Invalid key: {}", product_key);
	}
}
fn generate_product_key() -> String {
	let mut rng = rand::thread_rng();
	loop {
		let block_a: u32 = (0..=2)
			.map(|_| rng
				.gen_range('0'..='9')
				.to_digit(10)
				.unwrap_or_default()
			)
			.fold(0, |acc, digit| acc * 10 + digit);
		if !((3..=9).contains(&(block_a / 111)) && block_a % 111 == 0) {
			let block_b: u32 = (0..7)
			.map(|_| rng
				.gen_range('0'..='8')
				.to_digit(10)
				.unwrap_or_default()
			)
			.fold(0, |acc, digit| acc * 10 + digit);
			let block_b_check: Vec<u32> = block_b
				.to_string()
				.chars()
				.map(|c| c
					.to_digit(10)
					.unwrap()
				)
				.collect();
			if block_b_check
				.iter()
				.sum::<u32>() % 7 == 0 {
				let product_key: String = format!("{}-{}", block_a, block_b);
				return product_key;
			}
		}
	}
}
fn validate_product_key(product_key: &str) -> bool {
	let block_a: u32 = product_key[0..3]
		.parse()
		.unwrap_or_default();
	let block_b: u32 = product_key[4..]
    	.chars()
    	.filter_map(|c| c
			.to_digit(10)
		)
    	.sum();
	if (product_key
			.len() != 11
		) ||
		((3..=9)
			.contains(&(block_a / 111)) &&
			block_a % 111 == 0) ||
		(product_key
			.chars()
			.nth(3) != Some('-')
		) ||
		(block_b % 7 != 0) {
		return false;
	} else {
		return true;
	}
}