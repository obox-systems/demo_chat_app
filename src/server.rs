use std::net::TcpListener;

use actix_web::{
  delete,
  dev::Server,
  get, patch, post,
  web::{self, Data},
  App, HttpResponse, HttpServer, Responder,
};
use sqlx::{query, query_as, SqlitePool};

use crate::{
  data::{Message, MessageWithId},
  ws::{new_ws_server, ws_delete_message, ws_new_message, ws_update_message, Websocket},
};

#[delete("/message/{id}")]
async fn delete_message(
  id: web::Path<i64>,
  db: web::Data<SqlitePool>,
  ws: web::Data<Websocket>,
) -> impl Responder {
  let id = id.into_inner();
  match query!("DELETE FROM messages WHERE id = $1", id)
    .execute(db.as_ref())
    .await
  {
    Ok(_) => {
      ws.send(ws_delete_message(id))
        .expect("Failed to send data to the websocket");
      HttpResponse::Ok().finish()
    }
    Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
  }
}

#[patch("/message/{id}")]
async fn edit_message(
  id: web::Path<i64>,
  req: web::Json<Message>,
  db: web::Data<SqlitePool>,
  ws: web::Data<Websocket>,
) -> impl Responder {
  let id = id.into_inner();
  match query!(
    "UPDATE messages SET username = $1, message = $2 WHERE id = $3",
    req.username,
    req.message,
    id
  )
  .execute(db.as_ref())
  .await
  {
    Ok(_) => {
      ws.send(ws_update_message(id, req.into_inner()))
        .expect("Failed to send data to the websocket");
      HttpResponse::Ok().finish()
    }
    Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
  }
}

#[get("/messages")]
async fn get_messages(db: web::Data<SqlitePool>) -> impl Responder {
  match query_as!(MessageWithId, "SELECT * FROM messages")
    .fetch_all(db.as_ref())
    .await
  {
    Ok(messages) => HttpResponse::Ok().json(messages),
    Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
  }
}

#[post("/message")]
async fn new_message_handler(
  req: web::Json<Message>,
  db: web::Data<SqlitePool>,
  ws: web::Data<Websocket>,
) -> impl Responder {
  match query!(
    "INSERT INTO messages ( username, message ) VALUES ( $1, $2 )",
    req.username,
    req.message
  )
  .execute(db.as_ref())
  .await
  {
    Ok(changes) => {
      ws.send(ws_new_message(
        changes.last_insert_rowid(),
        req.into_inner(),
      ))
      .expect("Failed to send data to the websocket");
      HttpResponse::Ok().finish()
    }
    Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
  }
}

/// Binds to the address and serves the API.
pub async fn run(http: TcpListener, ws: &str, db: SqlitePool) -> anyhow::Result<Server> {
  let db = Data::new(db);
  let ws = Data::new(new_ws_server(ws).await?);
  let server = HttpServer::new(move || {
    App::new()
      .app_data(db.clone())
      .app_data(ws.clone())
      .service(new_message_handler)
      .service(get_messages)
      .service(edit_message)
      .service(delete_message)
  })
  .listen(http)?
  .run();
  Ok(server)
}
