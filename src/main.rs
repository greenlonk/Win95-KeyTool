/*Dependencies*/
use std::env;
use rand::Rng;

/*Code*/
fn main() {
	// Get arguments from terminal
	let args: Vec<String> = env::args()
		.collect();
	println!("Win95 Key Tool");
	// Check how many arguments are available
	if args.len() != 2 {
		eprintln!("Usage: {} <product key> / generate", args[0]);
		std::process::exit(1);
	}
	// Load argument into variable and check if it is called "generate"
	// Everything else will be checked if it is a valid product key
	let product_key: &str = args[1].as_str();
	if product_key == "generate" {
		println!("Windows 95 key: {}", generate_product_key());
	}
	else if validate_product_key(product_key) {
		println!("Valid key: {}", product_key);
	}
	else {println!("Invalid key: {}", product_key);}
}
fn generate_product_key() -> String {
	let mut rng = rand::thread_rng();
	loop {
		let block_a: String = (0..3)
			.map(|_| rng
				.gen_range('0'..='9'))
			.collect();
		if !["333", "444", "555", "666", "777", "888", "999"]
			.concat()
			.contains(&block_a) {
			let block_b: String = (0..7)
				.map(|_| rng.gen_range('0'..='8'))
				.collect();
			let block_b_numbers: Vec<u32> = block_b
				.chars()
				.map(|c| c
					.to_digit(10)
				.unwrap())
				.collect();
			if block_b_numbers.iter().sum::<u32>() % 7 == 0 {
				let product_key = format!("{}-{}", block_a, block_b);
				return product_key;
			}
		}
	}
}
fn validate_product_key(product_key: &str) -> bool {
	if product_key.len() != 11 {
		return false;
	}
	let block_a: usize = product_key[0..3].parse().unwrap_or_default();
    if block_a % 111 == 0 {
        if block_a >= 3 && block_a <= 9 {
			return false;
		}
	}
	if product_key.chars().nth(3) != Some('-') {
		return false;
	}
	let block_b: String = product_key[4..]
		.parse()
		.unwrap_or_default();
	let sum: usize = block_b
		.chars()
		.filter_map(|c| c
				.to_digit(10))
		.map(|d| d as usize)
		.sum();
	if sum % 7 != 0 {
		return false;
	}
	true
}