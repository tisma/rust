use std::io;
use rand::{distributions::Uniform, Rng};

fn bubble_sort_numbers(numbers: &mut Vec<i32>) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() - i - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort_numbers(numbers: &mut Vec<i32>) {
    for i in 0..numbers.len() {
        if let Some((min_index, _)) = numbers[i..].iter().enumerate().min_by_key(|&(_, v)| v) {
            numbers.swap(i, i + min_index);
        }
    }
}

fn insertion_sort_numbers(numbers: &mut Vec<i32>) {
    for i in 1..numbers.len() {
        let mut key = numbers[i];
        let mut j = i;
        while j > 0 && numbers[j - 1] > key {
            numbers[j] = numbers[j - 1];
            j = j - 1;
        }
        numbers[j] = key;
    }
}

fn quick_sort_numbers(numbers: &mut Vec<i32>) {
    quick_sort(numbers, 0, numbers.len() - 1);
}

fn partition(numbers: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = numbers[low];
    let mut count = 0;
    for i in low + 1..high + 1 {
        if numbers[i] <= pivot {
            count += 1;
        }
    }
    let mut pivotIndex = low + count;
    numbers.swap(pivotIndex, low);
    let mut i = low;
    let mut j = high;
    while i < pivotIndex && j > pivotIndex {
        while numbers[i] <= pivot {
            i += 1;
        }
        while numbers[j] > pivot {
            j -= 1;
        }
        if i < pivotIndex && j > pivotIndex {
            numbers.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    pivotIndex
}

fn quick_sort(numbers: &mut Vec<i32>, low: usize, high: usize) {
    if low >= high {
        return;
    }
    let pivot = partition(numbers, low, high);
    quick_sort(numbers, low, pivot - 1);
    quick_sort(numbers, pivot + 1, high);
}

fn print_usage() {
    println!("Choose selection algorithm: ");
    println!("1. Bubble Sort");
    println!("2. Selection Sort");
    println!("3. Insertion Sort");
    println!("4. Quick Sort");
    println!("0. Exit");
}

fn print_numbers(numbers: &Vec<i32>) {
    numbers.iter().for_each(|&um| { print!("{} ", um); });
    println!();
}

fn main() {
    let n: usize;
    let mut line = String::new();

    println!("Enter the number of elements: ");
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let n_result = line.trim().parse::<usize>();
    let n = match n_result {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Exiting...");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let mut numbers =  (0..n).map(|_| rng.gen_range(0..100)).collect();
    print_numbers(&numbers);
    print_usage();

    line.clear();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let option = line.trim().parse::<i32>().unwrap_or(-1);
    match option {
        1 => {
            let mut numbers = numbers.clone();
            bubble_sort_numbers(&mut numbers);
            print_numbers(&numbers);
        }
        2 => {
            let mut numbers = numbers.clone();
            selection_sort_numbers(&mut numbers);
            print_numbers(&numbers);
        }
        3 => {
            let mut numbers = numbers.clone();
            insertion_sort_numbers(&mut numbers);
            print_numbers(&numbers);
        }
        4 => {
            let mut numbers = numbers.clone();
            quick_sort_numbers(&mut numbers);
            print_numbers(&numbers);
        }
        0 => println!("Exiting..."),
        _ => print_usage(),
    }
}
