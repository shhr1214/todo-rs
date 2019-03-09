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

    println!("{:?}", cli);
}
