use rand::{Rng, RngExt};
pub enum Choice{
    AddTask,
    ShowTask,
    CompleteTask,
    DeleteTask,
    Quit
}

struct Task{
    id: i32,
    title: String,
    completed: bool,
}

pub struct TodoList{
    tasks: Vec<Task>,
}

impl TodoList{
    pub fn new() -> TodoList{
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title:String){
        let mut rng = rand::rng();
        let id = rng.random_range(1..=200);
        let new_task = Task{
            id,
            title,
            completed:false,
        }
        self.tasks.push(new_task);
        
    }
}

pub fn parse_option(num: i32)->Result<Choice, String>{
    match num {
        1 => Ok(Choice::AddTask),
        2 => Ok(Choice::ShowTask),
        3 => Ok(Choice::CompleteTask),
        4 => Ok(Choice::DeleteTask),
        5 => Ok(Choice::Quit),
        _ => Err("Not a valid option!".to_string())
    }
}

pub fn start_option(list: &mut TodoList, option:Choice, details:Option<String>){
    match option {
       Choice::AddTask => {
        match details {
            Some(title)=>list.add_task(title),
            None => println!("Can't be empty!"),
        }
       },
       Choice::ShowTask => show_task(),
       Choice::CompleteTask => complete_task(),
       Choice::DeleteTask => delete_task(),
       Choice::Quit => quit(),

    }
}