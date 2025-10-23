use std::io;

fn main() {
    let mut count: i32 = 0;

    println!("🧮 Welcome to the Counter Program!");
    println!("Commands: 'inc' to increment, 'dec' to decrement, 'reset' to reset, 'exit' to quit");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = input.trim();

        match command {
            "inc" => {
                count += 1;
                println!("Counter incremented ➡️ {}", count);
            }
            "dec" => {
                count -= 1;
                println!("Counter decremented ⬅️ {}", count);
            }
            "reset" => {
                count = 0;
                println!("Counter reset 🔁 {}", count);
            }
            "exit" => {
                println!("👋 Goodbye! Final count: {}", count);
                break;
            }
            _ => println!("❌ Unknown command. Try 'inc', 'dec', 'reset', or 'exit'."),
        }
    }
}
