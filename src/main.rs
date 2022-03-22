use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

mod utils;

#[derive(Deserialize)]
struct Request {
  grid: Vec<Vec<u8>>,
  turn: u8
}

#[derive(Serialize)]
struct Response {
  choice: u8
}

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello hans!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn solve_board(request: web::Json<Request>) -> Result<String> {
  let grid: Vec<Vec<u8>> = request.grid.clone();
  let turn: u8 = request.turn;
  let choice = utils::minimax(grid, turn);
  Ok(serde_json::to_string(&Response { choice }).unwrap())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .route("/solve-board", web::post().to(solve_board))
  })
  .bind(("0.0.0.0", 8088))?
  .run()
  .await
}
