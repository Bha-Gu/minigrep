use std::env;


fn main() {
	let args: Vec<String> = env::args().collect();

	let input1 = &args[1];
    println!("{:?}",input1);
}
