use core::task;
use std::io;
use todo_list::Choice;
use todo_list::TodoList;
use todo_list::parse_option;
use todo_list::start_option;

fn main() {
    let mut todo_list = TodoList::new();
    println!("Welcome to your Rust todo list!");
    println!("How can i help you today?");
    println!("1.Add Task\n2.Show Task\n3.Complete Task\n4.Delete Task\n5.Quit");
    let option: Choice = loop {
        let mut option_string: String = String::new();
        match io::stdin().read_line(&mut option_string) {
            Ok(_) => {}
            Err(_) => {
                println!("Error while reading the character");
                continue;
            }
        }
        let option_number = match option_string.trim().parse() {
            Ok(num) => {
                println!("You selected option {}", num);
                num
            },
            Err(_) => {
                println!("Not a valid number, try again");
                continue;
            }
        };

        match parse_option(option_number) {
            Ok(option) => break option,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    };

    match option {
        Choice::AddTask => {
            let task_title: String = String::new();
            match io::stdin().read_line(&mut task_title){
                Ok(_)=>start_option(&mut todo_list, Choice::AddTask, Some(String::from(task_title))),
                Err(_)=>{println!("Errore nell'inserimento del task")}
            }
        },
        Choice::ShowTask => start_option(&mut todo_list, option, details),
        Choice::CompleteTask => start_option(&mut todo_list, option, details),
        Choice::DeleteTask => start_option(&mut todo_list, option, details),
        Choice::Quit => start_option(&mut todo_list, option, details),
    }
    
}
