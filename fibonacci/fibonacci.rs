fn main(){
	let mut x = 0;
	let mut y = 1;
	let mut z = 0;

	for i in 0 .. 100{
		z = x + y;
		x = y;
		y = z;
		println!("{}", z);
	}
}
