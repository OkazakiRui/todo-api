use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

struct Todo {
    id: usize,
    title: String,
    completed: bool,
}
impl Todo {
    fn new(id: usize, title: String, completed: bool) -> Todo {
        Todo {
            id,
            title,
            completed,
        }
    }
}
struct AppState {
    todo_list: Mutex<Vec<Todo>>,
}
async fn index(data: web::Data<AppState>) -> String {
    let todo_list = data.todo_list.lock().expect("lock failed");

    todo_list
        .iter()
        .map(|todo| {
            format!(
                "id: {}\ntitle: {}\ncompleted: {}",
                todo.id, todo.title, todo.completed
            )
        })
        .collect::<Vec<String>>()
        .join("\n=========================\n")
}
// async fn createTask(data: web::Data<AppState>) -> String {
//     let mut todo_list = data.todo_list.lock().expect("lock failed");
// *counter += 1;

// format!("increment after number: {counter}")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_list = web::Data::new(AppState {
        todo_list: Mutex::new(vec![
            Todo::new(1, String::from("shop for groceries"), false),
            Todo::new(2, String::from("shop for groceries"), false),
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(todo_list.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
