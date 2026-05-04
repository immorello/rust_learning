pub enum Choice{
    AddTask,
    ShowTask,
    CompleTask,
    DeleteTask,
    Quit
}

pub fn parse_option(num: i32)->Result<Choice, String>{
    match num {
        1 => Ok(Choice::AddTask),
        2 => Ok(Choice::ShowTask),
        3 => Ok(Choice::CompleTask),
        4 => Ok(Choice::DeleteTask),
        5 => Ok(Choice::Quit),
        _ => Err("Not a valid option!".to_string())
    }
}