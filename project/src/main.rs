// line comment
/* 
   block
   comment
*/

fn print_sum(a: i8, b: i8) {
    println!("sum is {}", a + b);
}


fn main() {

	let a = true;
	let b: bool = true;

	let (x, y) = (1, 2);

	let mut z = 5;

	z = 6;

	let array = [1, 2, 3];
	let mut b = [1, 2, 3];

	let pa: [i32; 4] = [1, 2, 3, 4];

	let slice = &pa[2..4];

	// println!("slice = {}", slice);

	const N: i32 = 56;

	static S: i32 = 23;

	let fp: fn(i8, i8);

	fp = print_sum;

	fp(N as i8, S as i8);

	print_sum(z as i8, z as i8);

    println!("Hello, world!, {}, {}", N, S);


    let mut x = 1;
    while x <= 10 {
    	println!("Current value: {}", x);
    	x += 1;
    }

    for i in 0..10 {
    	print!("{} ", i);
    }
}
