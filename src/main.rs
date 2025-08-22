use actix_web::{get, App, HttpServer, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
    let todos = vec![
        Todo { id: 1, title: "buy beaver".to_string(), done: false },
        Todo { id: 2, title: "make a project".to_string(), done: true },
    ];
    web::Json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_todos) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
