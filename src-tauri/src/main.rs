#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sqlx::{
    sqlite::{Sqlite, SqlitePoolOptions, SqliteQueryResult},
    Pool, Row,
};

struct Database {
    pool: Pool<Sqlite>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Todo {
    description: String,
    is_complete: bool,
}

#[tauri::command]
async fn get_todos(database: tauri::State<'_, Database>) -> Result<Vec<Todo>, String> {
    match get_todos_repository(&database.pool).await {
        Ok(values) => Ok(values),
        Err(error) => Err(format!("Erro ao buscar Todos.")),
    }
}

#[tauri::command]
async fn add_todo(description: &str, database: tauri::State<'_, Database>) -> Result<Todo, String> {
    let todo = Todo {
        description: description.to_string(),
        is_complete: false,
    };
    println!("{:?}", todo);
    match add_todo_repository(&database.pool, &todo).await {
        Ok(_) => return Ok(todo),
        Err(error) => {
            println!("{:?}", error);
            return Err(format!("Erro ao salvar Todo."));
        }
    }
}

async fn add_todo_repository(
    pool: &Pool<Sqlite>,
    todo: &Todo,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let result = sqlx::query(
        format!(
            "INSERT INTO todo (description, is_complete) VALUES ('{}', {})",
            todo.description, todo.is_complete
        )
        .as_str(),
    )
    .execute(pool)
    .await?;
    Ok(result)
}

async fn get_todos_repository(pool: &Pool<Sqlite>) -> Result<Vec<Todo>, sqlx::Error> {
    let result = sqlx::query("SELECT * FROM todo")
        .fetch_all(pool)
        .await?;

    let todos: Vec<Todo> = result
        .iter()
        .map(|r| Todo {
            description: r.get::<String, _>("description"),
            is_complete: r.get::<bool, _>("is_complete"),
        })
        .collect();

    Ok(todos)
}

#[async_std::main]
async fn main() {
    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://data.db")
        .await {
            Ok(value) => value,
            Err(error) => panic!("Error {:?}", error)
        };

    tauri::Builder::default()
        .manage(Database { pool })
        .invoke_handler(tauri::generate_handler![add_todo, get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
