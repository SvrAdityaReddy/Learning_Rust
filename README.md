# Learning_Rust

## Rust Basics

Cargo is Rust's package installer, build tool, doc generator, etc

rustc is compiler here

To create Rust project

Do 

cargo new projectName

This would create a folder projectName with config file, src file main.rs

Variables are immutable by default in Rust to have safety, concurrency and speed

let bun = 1;
bun =2; //error

To resolve

rust --explain E0384

let mut bun = 1;

bun =2 ; // no error

const also even immutabler, convention in naming is use like MACROS.

These are const global immutable variables, inlined and type has always to be associated at compile time

const MACRO_ONE: i32 = 1;

The value must be a constant expression that can be deduced at compile time;

let are local scope constants

redifining a variable is allowed in same scope

let mut x =2;
let x =x;

even we can shadow variable to different type

let  x = 1
let x = 2.2

Memory Safety:

fn main() {
	let x: i32;
	if true {
		x = 2;
	}
	println!("{}", x); // Error as x is may not initialized
}

All uninitialized issues are resolved

functions are named as make_pair

can be at any place in the file

fn add(num1: i32, num2: i32) -> i32 {
	return num1+num2;
	// num1+num2 // tail expression
}

Use "cargo clippy" to catch common mistakes and improve your Rust code

Use "cargo clippy --fix" to apply few of them

Modules:


To package code and expose it as library

In cargo project create "src/lib.rs" file

add functions over there and declare them public

pub fn add(a: i32, b:i32) -> i32 {
	a+b
}

integer types values can have "_" in between values for readability

i8, u8(Byte)
f32, f64

type can provide as two ways

let x: u16 = 5;
let y: f32 = 3.2;

let x = 5u16 or let x = 5_u16
let y = 3.2f32 or let y = 3.2_f32

bool

char is always 4 bytes i.e UTF-32

Scalar types int, float, char, bool

Following are compound types

tuples

let info: (u32, u8, f32) = (1, 2, 3.2);

access using "." known as field access expression

let a = info.0;

maximum arity is 12 currently i.e maximum number of elements we can have

Arrays:

let buf = [0, 1, 3];
let buf = [0; 3]; // 0 is value, 3 is how many
let buf: [u32; 3] = [0, 1, 2];
buf[0]

Arrays are of max size 32

Use vec for having larger and Arrays are on stack by default

In conditionals expression always will be boolean, need not have paranthesis

msg =  if cnd1 {
	"cnd1"
} else {
	"cnd2"
};

"if" is an expression in rust so we can use as value to a variable
can have tail expressions

for num in [1,2 ,3].iter() {
println!("{}", num);
}

"loop" is unconditional loop, represents infinite loop

'outer: loop {
	if cnd {
		break 'outer;
	}
}

ranges 0..50
ranges 0..=50

string slice str } borrowed string value cannot be modified
String } value can be modified

by default assignment of string is borrowed string

let msg = "123".to_string();

let msg = String::from("123");


String have capacity apart from len

word.bytes();

word.chars(); // returns iterators
.nth(3) // for indexing into string, iterators facilitate this
