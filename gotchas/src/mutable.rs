fn get_sum(mut total: u32, mut i: u32) -> u32 {
    if i > 100 {
        return total;
    }

    total += i;
    i += 1;
    get_sum(total, i)
}

fn main() {
    let total = 0;
    let total = get_sum(total, 1);
    println!("{}", total);
}

