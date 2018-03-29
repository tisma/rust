struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
	let _a: Vec<i32> = Vec::new();
	let mut b: Vec<i32> = vec![5, 4, 3, 2, 1, 0];    

	b.push(523);

	println!("{:?}", b);

	let mut e: Vec<i32> = Vec::with_capacity(10);
	println!("length: {}, capacity: {}", e.len(), e.capacity());

	for i in 0..10 {
		e.push(i);
	}

	println!("length: {}, capacity: {}", e.len(), e.capacity());

	e.push(11);
	println!("length: {}, capacity: {}", e.len(), e.capacity());
	
	let x = e.pop();
	println!("x = {}, length: {}, capacity: {}", x.unwrap(), e.len(), e.capacity());

	let mut v = vec![1,2 ,23, 43,243,423];

	for i in &v {
		println!("A reference to {}", i);
	}

	for i in &mut v {
		println!("A mutable reference to {}", i);
	}

	for i in v {
		println!("Take ownership of the vector and its element {}", i);
	}

	let black = Color { red: 0, green: 0, blue: 0 };
	println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue);
	
	let Color { red: r, green: g, blue: b } = get_midnightblue_color();
	println!("Midnight blue = rgb({}, {}, {})", r, g, b);

}

fn get_midnightblue_color() -> Color {
    Color {red: 25, green: 25, blue: 112 }
}