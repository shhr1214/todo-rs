use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug)]
struct Cli {
    command: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TaskList {
    size: u64,
    tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: String,
}

fn main() -> Result<(), Error> {
    let command = std::env::args().nth(1).expect("no command given");
    let name = std::env::args().nth(2).expect("no name given");

    let cli = Cli {
        command: command,
        name: name,
    };

    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<(), Error> {
    if cli.command == "list".to_string() {
        let tasks = read_tasklist("./test/sample.json".to_string())?;
        show_tasklist(&tasks);
    } else if cli.command == "add".to_string() {
        // let task = Task { name: cli.name };
    }
    Ok(())
}

fn read_tasklist(filename: String) -> Result<TaskList, Error> {
    let contents = fs::read_to_string(filename)?;
    let list = serde_json::from_str(&contents)?;
    Ok(list)
}

fn show_tasklist(tasklist: &TaskList) {
    for ref task in &tasklist.tasks {
        show_task(task);
    }
}

fn show_task(task: &Task) {
    println!("{}", task.name);
}
