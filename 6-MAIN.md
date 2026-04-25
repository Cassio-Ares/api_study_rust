 # final main

 ````
use actix_web::{get, App, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;


// `mod` diz ao Rust: "esse módulo existe e faz parte do projeto"
// sem declarar aqui -> o compilador não encontra os arquivos
mod models;      // src/models/     -> structs Payment, NewPayment...
mod repositories;// src/repositories/-> queries SQL
mod services;    // src/services/   -> regras de negócio
mod handlers;    // src/handlers/   -> funções dos endpoints
mod routes;      // src/routes/     -> mapeamento de URLs
mod validators;  // src/validators/ -> regras de validação
// futuro terá um mod test

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello world!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))

            // Rota de teste — ainda registrada
            .service(hello)

            // NOVIDADE — registra todas as rotas de payment de uma vez
            // Delega a configuração para o arquivo routes/payment_routes.rs
            // sem isso -> nenhum endpoint de payment funciona
            .configure(routes::payment_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
 ````