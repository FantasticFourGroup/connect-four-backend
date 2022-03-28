use actix_web::{web, get, post, http, App, HttpResponse, HttpServer, Responder, Result};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

mod utils;

#[derive(Deserialize)]
struct Request {
  grid: Vec<Vec<usize>>,
  depth: usize,
  turn: usize
}

#[derive(Serialize)]
struct Response {
  choice: usize
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
  let grid: Vec<Vec<usize>> = request.grid.clone();
  let depth: usize = request.depth;
  let turn: usize = request.turn;
  let choice = utils::solve_board(grid, depth, turn);
  match choice {
    Some(choice) => Ok(format!("{}", choice)),
    None => Ok(format!("None"))
  }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let cors = Cors::default()
      .allow_any_origin()
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    App::new()
      .wrap(cors)
      .service(hello)
      .service(echo)
      .service(web::resource("/solve-board").route(web::post().to(solve_board)))
  })
  .bind(("0.0.0.0", 8088))?
  .run()
  .await
}
