use std::io;

fn main() {
    println!("Simple fibonacci");

    loop {
      let mut input = String::new();

      println!("Enter a number or [q]uit: ");

      io::stdin().read_line(&mut input).expect("Failed to read line");

      if input.trim() == "quit" || input.trim() == "q" {
        break;
      }

      let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };
      
      println!("{}", fib(n));
    }
}

fn fib(n: u32) -> u32 {
    match n {
      0 => 1,
      1 => 1,
      _ => fib(n - 1) + fib(n - 2),
    }
}
