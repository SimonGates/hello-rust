use colored::*;

fn main() {
    println!("Hello, world!");
    for n in 1..1_001 {
        if n % 3 == 0 {
            println!("{number} FIZZ", number = n);
        }
        if n % 6 == 0 {
            println!("{number} RIZZ", number = n);
        }
        if n % 6 == 0 && n % 7 == 0 {
            eprintln!("{}", "67".bold().red());
        }
    }
}
