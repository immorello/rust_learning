use rand::{ RngExt};
pub enum Choice{
    AddTask,
    ListTasks,
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
        };
        self.tasks.push(new_task);
        println!("Task inserted with id {}", id.to_string());
        
    }

    fn list_tasks(&self){
        println!("Here's the list of tasks");
        for task in &self.tasks{
            println!("Task {} - Title: {} - State: {}", task.id.to_string(), task.title, task.completed.to_string())
        }
    }

    fn show_task(&self, id:i32)->Option<&Task>{
       let task_to_show = self.tasks.iter().find(|task| task.id == id);
       task_to_show
    }

    pub fn complete_task(&mut self, id:i32){
        let task_to_complete = self.tasks.iter_mut().find(|task| task.id == id);
        match task_to_complete{
            Some(task_to_complete)=>{
                task_to_complete.completed = true;
                println!("Task {} set to completed", task_to_complete.title);
            },
            None => println!("No task found with id {}", id.to_string()),
        }
    }

    pub fn delete_task(&mut self, id:i32){
        let task_to_delete_index = self.tasks.iter().position(|task| task.id == id);
        match task_to_delete_index{
            Some(index)=>{
                self.tasks.remove(index);
                println!("Task deleted");
            },
            None => println!("No task found with id {}",id.to_string()),
        }
    }
}

pub fn parse_option(num: i32)->Result<Choice, String>{
    match num {
        1 => Ok(Choice::AddTask),
        2 => Ok(Choice::ListTasks),
        3 => Ok(Choice::ShowTask),
        4 => Ok(Choice::CompleteTask),
        5 => Ok(Choice::DeleteTask),
        6 => Ok(Choice::Quit),
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
       Choice::ListTasks =>{
        list.list_tasks();
       },
       Choice::ShowTask => {
        match details {
            Some(str_id)=>{
                let id: i32 = match str_id.trim().parse(){
                    Ok(num_id)=> num_id,
                    Err(_)=>{println!("Error while parsing id!");
                    return;}
                };
                match  list.show_task(id){
                    Some(task_to_show)=>println!("Il tuo task ha id:{}, titolo:{} ed stato:{}",task_to_show.id.to_string(),task_to_show.title,task_to_show.completed.to_string()),
                    None => println!("No task found with id {}",id.to_string())
                };
            },
            None => println!("Id can't be empty"),
        }
       },
       Choice::CompleteTask => {
        match details {
            Some(str_id)=>{
                let id: i32 = match str_id.trim().parse() {
                    Ok(num_id)=>num_id,
                    Err(_)=>{
                        println!("Error while parsig id!");
                        return;
                    }
                };
                list.complete_task(id);
            }
            None => println!("Id can't be empty"),
        }
       },
       Choice::DeleteTask => {
            match details {
                Some(str_id)=>{
                    let id:i32 = match str_id.trim().parse(){
                        Ok(num_id)=>num_id,
                        Err(_)=>{
                            println!("Error while parsing id!");
                            return;
                        }
                    };
                    list.delete_task(id);
                },
                None => println!("Id can't be empty"),
            }
       },
       Choice::Quit => {
        println!("Bye bye!");
        std::process::exit(0)},

    }
}