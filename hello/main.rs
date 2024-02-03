use std::env;

use chrono::{DateTime, Datelike, Local};
use task_manager::build;
use::task_manager::get_old_task;
use::chrono;

fn main() {
    #[allow(dead_code)]
    let args : Vec<String>= env::args().collect();
    #[allow(dead_code)]
    let all_task = get_old_task();

    // build(&args, &all_task);
    println!(" {}",chrono::Utc::now().day());
    println!(" {}",chrono::offset::Local::now());

        
}
