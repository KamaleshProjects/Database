use std::io;

fn main() {
    println!("Hey!, this is SQLWaffle!");

    // repr -> read execute print loop
    loop {
        println!("> ");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        command = command.trim().to_string();
        println!("You entered the command: '{}' ", command);
    }
}
