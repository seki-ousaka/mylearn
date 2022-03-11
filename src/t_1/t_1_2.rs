pub fn t_1_2_do() -> i32 {
	let n1: i32 = 987;
	let n2: i32 = 123;
	let n3: i32 = -654;
	let n4: i32 = 456;
	let n5: f32 = 10.0;
	let n6: f32 = 2.0;

	// the four basic arithmetic operations 
	println!("addition(987 + 123):{}", &n1 + &n2);
	println!("subtraction(987 - 123):{}", &n1 - &n2);
	println!("multiplication(987 * 123):{}", &n1 * &n2);
	println!("division(987 / 123 [i32]):{}", &n1 / &n2);

	// mod
	println!("mod(987 % 123 [i32]):{}", &n1 % &n2);

	// abs
	println!("abs(-654):{}", &n3.abs());
	println!("abs2(456):{}", &n4.abs());

	// pow
	println!("pow(10^2 [f32]):{}", &n5.powf(n6));

	// sqrt
	println!("sqrt(√2.0){}", &n6.sqrt());

	// return
	0
}