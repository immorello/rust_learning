use std::io;

fn main() {
    println!("Welcome to your Rust todo list!");
    println!("How can i help you today?");
    println!("1.Add Task\n2.Show Task\n3.Complete Task\n4.Delete Task\n5.Quit");
    let option_number: i32 = loop {
        let mut option_string: String = String::new();
        match io::stdin().read_line(&mut option_string) {
            Ok(_) => {}
            Err(_) => {
                println!("Error while reading the character");
                continue;
            }
        }
        match option_string.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Not a valid number, try again"),
        }
    };

    println!("You selected option {}", option_number);
}
