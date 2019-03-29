extern crate libc;

use libc::{c_char, printf, scanf, EOF};

fn main() {
    unsafe {
        let mut num = 0;
        let mut sum = 0;
        while (scanf(b"%d\0" as *const u8 as *const c_char, &mut num) != EOF) {
            sum += num;
        }
        printf(b"Sum: %d\n\0" as *const u8 as *const c_char, sum);
    }
}

