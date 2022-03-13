use std::env;

pub fn t_1_4_do() -> i32 {
    let args: Vec<String> = env::args().collect();


	if args.len() <= 1 {
		eprintln!("error: not enough args");
		return 0
	}

	let input_arg :i32 = match args[1].parse() {
		Ok(n) => {
			n
		},
		Err(_) => {
			eprintln!("error: this argument not an integer {}" , args[1]);
			0
		},
	};

	2 * input_arg + 100
}

