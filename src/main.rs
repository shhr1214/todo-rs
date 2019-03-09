#[derive(Debug)]
struct Task {
    name: String,
}

fn main() {
    let task = Task {
        name: "first task".to_string(),
    };

    println!("{:?}", task);
}
