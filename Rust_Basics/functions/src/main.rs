use functions::add_two_numbers;

fn main() {
	let a=1;
	let b=2;
	let c=3;
	let ad = add(a, b, c);
	let mul = multiply(a, b, c);
	println!("Addition is: {}", ad);
	println!("Multiplication is: {}", mul);
	println!("Addition of two numbers: {}", add_two_numbers(a, b));
}

fn add(a: i32, b: i32, c: i32) -> i32 {
	a+b+c
}

fn multiply(a: i32, b:i32, c: i32) -> i32 {
	return a*b*c;
}

