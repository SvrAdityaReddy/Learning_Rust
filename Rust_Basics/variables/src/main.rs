fn main() {
	let mut missiles = 8;
	let ready = 2;
	let mut missiles: i32 = 8;
	let ready: i32 = 2;
	let (_missiles1, ready2): (i32, i32) = (8, 2); // bind all variables at once using a pattern

	// unused variables throw compilation warning
	// to not show warning we can add "_" before variable name
    println!("Firing {} of {} missiles...", ready, missiles);
	println!("{} missiles left", missiles-ready);
	missiles = missiles - ready;
	println!("{} missiles left", missiles);
}
