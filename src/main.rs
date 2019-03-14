use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufWriter, Write};

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

impl TaskList {
    fn add(&mut self, task: Task) {
        self.size = self.size + 1;
        self.tasks.push(task);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    let filename = "./test/sample.json";
    let mut tasks = read_tasklist(filename)?;
    if cli.command == "list".to_string() {
        show_tasklist(&tasks);
    } else if cli.command == "add".to_string() {
        let task = Task { name: cli.name };
        tasks.add(task);
        save_tasklist(filename, &tasks)?;
    }
    Ok(())
}

fn read_tasklist(filename: &str) -> Result<Box<TaskList>, Error> {
    let contents = fs::read_to_string(filename)?;
    let list = serde_json::from_str(&contents)?;
    Ok(list)
}

fn save_tasklist(filename: &str, tasklist: &TaskList) -> Result<(), Error> {
    let mut f = BufWriter::new(fs::File::create(filename)?);
    let json = serde_json::to_string(tasklist)?;
    f.write(json.as_bytes())?;
    Ok(())
}

fn show_tasklist(tasklist: &TaskList) {
    for ref task in &tasklist.tasks {
        show_task(task);
    }
}

fn show_task(task: &Task) {
    println!("{}", task.name);
}
