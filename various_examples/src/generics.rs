struct Point<T> {
	x: T,
	y: T,
}

fn main() {
    let point_a = Point { x: 0, y: 0 };
    let point_b = Point { x: 0.0, y: 0.0 };

    println!("Point A: ({}, {})", point_a.x, point_a.y);
    println!("Point B: ({:.2}, {:.2})", point_b.x, point_b.y);
}

