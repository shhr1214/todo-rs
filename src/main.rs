use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug)]
struct Cli {
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
    let name = std::env::args().nth(1).expect("no name given");

    let cli = Cli { name: name };

    run(cli)?;
    Ok(())
}

fn run(_: Cli) -> Result<(), Error> {
    let tasks = read_tasklist("./test/sample.json".to_string())?;
    // let task = Task { name: cli.name };

    show_tasklist(&tasks);
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
