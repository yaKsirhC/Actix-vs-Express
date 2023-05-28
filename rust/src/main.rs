mod sql_api;
use std::*;
use actix_web::{App, get, post ,HttpServer, web, HttpResponse};
use actix_files::Files;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use uuid::Uuid;
use tokio::io::AsyncWriteExt as _;
use tokio::fs;

#[get("/create-user/{num}")]
async fn create_nusers(path: web::Path<u16>)-> HttpResponse {
  let number = path.into_inner();
  for _ in 1..number+1 {
    let _ = sql_api::create_user().unwrap();
  }
  HttpResponse::Ok().finish()
}
#[get("/create-post/{uid}/{num}")]
async fn create_nposts(path: web::Path<(u16, u16)>) ->  HttpResponse{
  let (uid, post_num) = path.into_inner();

  for _ in 1..post_num+1 {
    let _ = sql_api::create_post(uid).unwrap();
  }
  HttpResponse::Ok().finish()
}

#[get("/heavy/{iter}")]
async fn heavy(path: web::Path<u32>) -> HttpResponse {
  let iter = path.into_inner();
  for _ in 1..100_000*iter+1 {
    let _ = 1+1;
  }

  HttpResponse::Ok().finish()
}

#[post("/upload")]
async fn upload_file(mut buffer: Multipart) -> HttpResponse{

  loop {
    if let Ok(Some(mut field)) = buffer.try_next().await {
      let mut saved_file = fs::File::create("./uploads/".to_string() + &Uuid::new_v4().to_string()).await.unwrap();
      while let Ok(Some(chunk)) = field.try_next().await {
        let _ = saved_file.write_all(&chunk).await.unwrap();
      }
    }
    else { break; }
  }

  HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| 
        App::new()
        .service(create_nusers)
        .service(create_nposts)
        .service(upload_file)
        .service(heavy)
        .service(Files::new("/files", "./files"))
        .service(actix_files::Files::new("/", "./dist/").index_file("index.html"))
        )
        .bind(("127.0.0.1", 9001))?
        .run()
        .await
}
