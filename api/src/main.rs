use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

// Todo struct
#[derive(Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

// Todoのリクエスト用
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}
// Updateのリクエスト用
#[derive(Deserialize)]
struct UpdateTodo {
    title: String,
    completed: bool,
}

// データベース接続の設定
async fn get_db_pool() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url).await.unwrap()
}

// Todoを全て取得
async fn get_todos(pool: web::Data<SqlitePool>) -> impl Responder {
    let todos = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id!", title as "title!", completed FROM todos"#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(todos)
}

// 新しいTodoを作成
async fn create_todo(
    pool: web::Data<SqlitePool>,
    todo_data: web::Json<CreateTodo>,
) -> impl Responder {
    let new_id = Uuid::new_v4().to_string();

    // データベースに新しいTodoを挿入
    sqlx::query!(
        "INSERT INTO todos (id, title, completed) VALUES (?, ?, ?)",
        new_id,
        todo_data.title,
        false,
    )
    .execute(pool.get_ref())
    .await
    .unwrap();

    // 新しく作成されたTodoを作成して返す
    let new_todo = Todo {
        id: new_id.clone(),
        title: todo_data.title.clone(),
        completed: false,
    };

    // JSONレスポンスとして新しいTodoを返す
    HttpResponse::Created().json(new_todo)
}

// Todoの更新処理
async fn update_todo(
    pool: web::Data<SqlitePool>,
    todo_id: web::Path<String>,
    todo_data: web::Json<UpdateTodo>,
) -> impl Responder {
    let id = todo_id.into_inner();
    sqlx::query!(
        "UPDATE todos SET title = ?, completed = ? WHERE id = ?",
        todo_data.title,
        todo_data.completed,
        id,
    )
    .execute(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().body("Todo updated")
}

// Todoの削除処理
async fn delete_todo(pool: web::Data<SqlitePool>, todo_id: web::Path<String>) -> impl Responder {
    let id = todo_id.into_inner();
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().body("Todo deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // 必要に応じて変更
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
