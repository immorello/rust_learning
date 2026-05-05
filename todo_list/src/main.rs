use std::io;
use todo_list::Choice;
use todo_list::TodoList;
use todo_list::parse_option;
use todo_list::start_option;

fn main() {
    let mut todo_list = TodoList::new();
    println!("Welcome to your Rust todo list!");
    loop {
        println!("\nHow can i help you?");
        println!("1.Add Task\n2.List Tasks\n3.Show Task\n4.Complete Task\n5.Delete Task\n6.Quit");
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
                }
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
                println!("Inserisci il titolo del task da aggiungere");
                let mut task_title: String = String::new();
                match io::stdin().read_line(&mut task_title) {
                    Ok(_) => start_option(
                        &mut todo_list,
                        Choice::AddTask,
                        Some(String::from(task_title)),
                    ),
                    Err(_) => {
                        println!("Errore nell'inserimento del task")
                    }
                }
            },
            Choice::ListTasks => start_option(&mut todo_list, Choice::ListTasks, None),
            Choice::ShowTask => {
                println!("Inserisci l'id del task da cercare");
                let mut task_id: String = String::new();
                match io::stdin().read_line(&mut task_id) {
                    Ok(_) => start_option(
                        &mut todo_list,
                        Choice::ShowTask,
                        Some(String::from(task_id)),
                    ),
                    Err(_) => {
                        println!("Errore nella ricerca del task")
                    }
                }
            },
            Choice::CompleteTask => {
                println!("Inserisci l'id del task da completare");
                let mut task_id: String = String::new();
                match io::stdin().read_line(&mut task_id) {
                    Ok(_) => start_option(
                        &mut todo_list,
                        Choice::CompleteTask,
                        Some(String::from(task_id)),
                    ),
                    Err(_) => {
                        println!("Errore nel completamento del task")
                    }
                }
            },
            Choice::DeleteTask => {
                println!("Inserisci l'id del task da eliminare");
                let mut task_id: String = String::new();
                match io::stdin().read_line(&mut task_id) {
                    Ok(_) => start_option(
                        &mut todo_list,
                        Choice::DeleteTask,
                        Some(String::from(task_id)),
                    ),
                    Err(_) => {
                        println!("Errore nella cancellazione del task")
                    }
                }
            },
            Choice::Quit => start_option(&mut todo_list, Choice::Quit, None),
        }
    }
}
