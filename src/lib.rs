use std::error::Error;
use actix_web::{HttpServer, dev::Server, web, Responder, App, HttpResponse, http::StatusCode, get, post, patch, delete};
use rusqlite::Connection;
use tokio::{sync::Mutex};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
  username: String,
  message: String
}

#[delete("/message/{id}")]
async fn delete_message(id: web::Path<String>, db: web::Data<Mutex<Connection>>) -> HttpResponse
{
  let id = id.parse::<usize>().unwrap();
  let res = db.lock().await.execute(
    "DELETE FROM messages WHERE id = ?1", 
    [id]
  );

  match res {
    Ok(_) => HttpResponse::build(StatusCode::OK).finish(),
    Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(),
  }
}

#[patch("/message/{id}")]
async fn edit_message(id: web::Path<String>, req: web::Json<Message>, db: web::Data<Mutex<Connection>>) -> HttpResponse {
  let id = id.parse::<usize>().unwrap();

  let res = db.lock().await.execute(
    "UPDATE messages SET message = '?1' WHERE id = ?2",
    (&req.message, &id)
  );

  match res {
    Ok(_) => HttpResponse::build(StatusCode::OK).finish(),
    Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(),
  }
}

#[get("/messages")]
async fn get_messages(db: web::Data<Mutex<Connection>>) -> HttpResponse {
  let lock = db.lock().await;
  let mut stmt = match lock.prepare(
    "SELECT id, username, message FROM messages LIMIT 50"
  ){
    Ok(stmt) => stmt,
    Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish()
  };
  let iter = stmt.query_map([], |row| {
    Ok( Message { username: row.get(1)?, message: row.get(2)? } )
  }).unwrap();
  let mut messages = vec![];
  for message in iter {
    messages.push(message.unwrap());
  }

  HttpResponse::build(StatusCode::OK).json(messages)
}

#[post("/message")]
async fn new_message_handler(req: web::Json<Message>, db: web::Data<Mutex<Connection>>) -> impl Responder {
  if let Ok(_) = db.lock().await.execute(
    "INSERT INTO messages (username, message) VALUES (?1, ?2)", 
      (&req.username, &req.message)) {
        HttpResponse::build(StatusCode::OK)
      } else {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
      }
}

fn create_temp_db() -> rusqlite::Result<Connection> {

  let conn = Connection::open_in_memory()?;

  conn.execute(
    "CREATE TABLE messages (
      id        INTEGER PRIMARY KEY,
      username  TEXT NOT NULL,
      message   TEXT NOT NULL
     )", ())?;
  Ok(conn)
}

pub fn run(http: std::net::TcpListener) -> Result<Server, Box<dyn Error>>
{
  let conn = create_temp_db()?;

  let data = web::Data::new(
    Mutex::new(conn)
  );
  let server = HttpServer::new(
    move ||{
      App::new()
        .app_data(data.clone())
        .service(new_message_handler)
        .service(get_messages)
        .service(edit_message)
        .service(delete_message)
    }
  ).listen(http)?.run();
  Ok(server)
}