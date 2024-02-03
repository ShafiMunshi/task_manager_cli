use std::{ fs::{self, File}, process::Command};

use chrono::{DateTime, Local};


pub struct TaskManger {
    _task_list: Vec<Task>,
    // task_status: HashMap<String, u8>,
}
#[derive(Debug)]// TODO: if i use Clone here , using clone method Vec<Task> becomes [Task]
#[allow(dead_code)]
pub struct Task {
    name: String,
    id: usize,
    due_date: String,
    priority: String,
    categories: String,
}

impl Task {
    pub fn new_task(name: String, all_task_len:usize) -> Task {
        // let all_task_len=
        // let current_date = get_current_date();
        Task {
            name,
            id: all_task_len+1,
            due_date: String::new(),
            priority: String::new(),
            categories: String::new(),
        }
    }
}

pub fn build(args: &[String], task_list :& Vec<Task>) {
    println!("arguments:  {:?}",args);
    if args[2] == "list" {
        // show list item
        show_task_list(&task_list);
    }
    else if args[1] == "add" && !args[2].is_empty(){
        //add a new taskt to file 
       add_task(&args[2], &task_list)
    }
     else if args[1] == "complete" && !args[2].is_empty() {
        // mark task as complete using id
        
    } else if args[1] == "delete" && !args[2].is_empty() {
        // delet that id
        delete_task(&args[2], &task_list);
    } else if args[2] == "tag" && !args[3].is_empty() && !args[4].is_empty() {
        // set the tag using tag id
    } else if args[2] == "search" && !args[3].is_empty() {
        // search the task using task name
    } else if args[2] == "save" {
        // save all the task in a file
    } else {
        // throw an error
    }
}

pub fn get_old_task() -> Vec<Task>{//
    // get saved task list
    let mut task_list= Vec::new();
    let file_path= "task_list.txt";
    let task_contents= fs::read_to_string(file_path).expect("File not found");

    for line in task_contents.lines(){
        //println!("from file:  {}",line);
        let (mut name,mut  id, mut tag, mut due, mut priority)=("fas".to_string(),100,"asd".to_string(), "d".to_string(), "fas".to_string());
        for (indx, word) in line.split_whitespace().enumerate(){
            if indx==0{
                if let Some(first_char) = word.chars().next() {
                    //TODO: Convert the character to a u8, but how it works?
                    id = first_char as usize;// it is storing as ASCII code
                    
                    // println!("Converted u8: {}", result_u8);
                } else {
                    println!("The string is empty.");
                }
            }else if indx==1{
                name= word.to_string();
            }else if indx==2{
                tag=word.to_string();
            }else if indx==3{
                priority= word.to_string();
            }else if indx==4{
                due=word.to_string()
            }
        }

        task_list.push(Task { name, id, due_date: due, priority, categories: tag });
    }

    task_list


}

pub fn show_task_list(task_list: &Vec<Task>) {
    for task in task_list{
        println!("{:?}",task);
    }
}

#[allow(dead_code)]
pub fn delete_task(id: &str, task_list :& Vec<Task>){
    // delete task
    
}

pub fn add_task(name: &str, all_task: &Vec<Task>){
    let all_task_len = all_task.len();
    println!(" Calling adding task func");
    // create a new task
    let task = Task::new_task(name.to_string(), all_task_len);// TODO: getting error to create a vector
    let mut v = all_task.clone();
    // let mut v=vec![];
    v.push(task);
    //let arr: [&str;2]= v.into_iter().collect();// TODO: converting vector to an array
    let task_strings: Vec<String> = all_task.iter().map(|task| format!("{:?}", task)).collect();// TODO: here is another
    // save it to file

    let log_name = format!("tasklist.txt",);
    let log = File::create(log_name).expect("failed to open log");

    let mut cmd = Command::new("echo")
        .args(task_strings)
        .stdout(log)
        .spawn()
        .expect("failed to start echo");

    cmd.wait().expect("failed to finish echo");

    let hello = cmd.stdout;
    println!(" Hello:    {:?}",hello);
}


pub fn add_due_status(task_list :& Vec<Task>, status: &str, id: usize){
    // mark due status as completed or mark due date
}

// pub fn get_current_date()->String{
//     let current_local: DateTime<Local> = Local::now();
//     let custom_format = current_local.format("%Y-%m-%d");
//     println!("{}", custom_format); // Outputs: 2023-10-03 16:41:00
//     custom_format
// }


// pub fn convert_id(id: &str)->usize{
//     if let Some(first_char) = id.chars().next() {
//         //TODO: Convert the character to a u8, but how it works?
//        return first_char as usize;// it is storing as ASCII code
        
//         // println!("Converted u8: {}", result_u8);
//     }
// }

