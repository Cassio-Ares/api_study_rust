use actix_web::{get, App, HttpServer, Response};


#[get ("/hello")]
async fn hello()-> impl Responder{
    "Hello world"
}

#[actix_web::main]
async fn main(){
 HttpServer::new(|| {
     App::new()
         .service(hello)
 })
 .bind(("127.0.0.1", 8080))?
 .run()
 .await
}