use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;

fn index() -> impl Responder {
    HttpResponse::Ok().body("My Todo App!")
}

fn list_tasks(_req: HttpRequest) -> impl Responder {
    let mut tasks = Vec::new();

    tasks.push(Task::new("test1".into()));
    tasks.push(Task::new("test2".into()));
    tasks.push(Task::new("test3".into()));
    tasks.push(Task::new("test4".into()));

    Tasks { tasks: tasks }
}

fn get_tasks(req: HttpRequest) -> impl Responder {
    let id: &str = req.match_info().get("id").unwrap();
    let id = id.to_owned();
    println!("{}", id);

    Task::new("test1".into())
}

#[derive(Clone, Debug, Serialize)]
struct Task(todo_rs::Task);

impl Task {
    fn new(name: String) -> Self {
        Task(todo_rs::Task::new(name))
    }
}

impl Responder for Task {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Serialize)]
struct Tasks {
    tasks: Vec<Task>,
}

impl Responder for Tasks {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/tasks", web::get().to(list_tasks))
            .route("/tasks/{id}", web::get().to(get_tasks))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
