#[derive(Debug)]
struct Cli {
    name: String,
}

#[derive(Debug)]
struct Task {
    name: String,
}

fn main() {
    let name = std::env::args().nth(1).expect("no name given");

    let cli = Cli { name: name };

    run(cli);
}

fn run(cli: Cli) {
    let task = Task { name: cli.name };
    println!("{:?}", task);
}
