use std::env;

pub fn t_1_1_do(){
	let args: Vec<String> = env::args().collect();

	let mut sum: i32 = 0;
	for arg in &args[1..] {
		let num: i32 = match arg.parse() {
			Ok(n) => {
				n
			},
			Err(_) => {
				eprintln!("error: this argument not an integer {}" , arg);
				return;
			},
		};
		sum = &sum + num;		
	}

	println!("{}", sum);
} 