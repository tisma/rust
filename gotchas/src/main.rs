fn main() {
    let mut i: isize = 1;
    let mut j: isize = 2;
    foo(&mut i, &mut j);
    println!("{} {}", i, j);
}

fn foo<'a>(mut i: &'a mut isize, j: &'a mut isize) {
    *i *= 10;
    i = j;
    *i *= 10;
}

