use std::io;

fn main() {
    println!("Calculate reps in a given round");

    loop {
        let mut input = String::new();

        println!("Enter a number or [q]uit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "quit" || input.trim() == "q" {
            break;
        }

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", reps_for_round(n));
    }
}

fn reps_for_round(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => n + reps_for_round(n - 1),
    }
}
