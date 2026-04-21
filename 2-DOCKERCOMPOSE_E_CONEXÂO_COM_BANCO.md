# Docker compose e conexão com banco

## docker-compose.yml
````
version: '3.8'

services:
   db:
    image: postgres:13
    environment:
      POSTGRES_DB: payment_db
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    
    app:
    build:
      context: ..           # raiz do projeto
      dockerfile: docker/Dockerfile
    restart: always
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgres://user:password@db:5432/payment_db
    depends_on:
      - db
````

## cargo.toml

 - add:
````
  tokio = { version = "1", features = ["full"] }
  sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
  dotenv = "0.15"
````

## main.rs e conexão com banco

````
use actix_web::{get, App, HttpServer, Responder};

// Importa SQLx para trabalhar com banco de dados
// ´PgPoolOptions´ -> configuração do pool de conexões
// ´Pool´ -> o pool de conexão 
// ´Postgres´ -> tipo de representação do banco postgres
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// Importa do dotenv, que lê variáveis de um arquivo .env
use dotenv::dotenv;

// Importa o módulo `env` da biblioteca padrão do Rust
// Usado para ler variáveis de ambiente do sistema
use std::env;

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
//sdt::io::Result<()> -> agora a main retorna um Result
// Significa que a função pode retornar um erro de I/O (ex: porta ocupada)
async fn main()-> std::io::Result<()> {
  
  // Carrega as variáveis do arquivo ".env" para o ambiente
  // "ok()" ignora silenciosamente se o arquivo não existir
  dotenv().ok();

  // ´env::var("DATABASE_URL")´ -> Lê a variável de ambiente "DATABASE_URL"
  // ´expect´ -> trava o programa com uma mensagem se a variável não existir
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  // Crio o Pool de conexão com o banco de dados
  let pool = PgPoolOptions::new()
        .max_connections(5) // Limite de 5 conexões simultâneas com o
        .connect(&database_url) // Conecta usando a URL ".env"
        .await // Aguarda a conexão ser estabelecida
        .expect("Failed to create pool."); // Trava se não conseguir 

 // ´move´ -> tranfere a posse do pool para dentro do closure (||)  
 // Importante pois cada thread precisa ter sua própria cópia do pool
  HttpServer::new(move || {
        App::new()
            // compartilha o pool com todas as rotas da aplicação
            // `Data::new` -> empacota o pool com todas as rotas da aplicação
            // ´.clone´ -> cria uma copia do pool para cada thread 
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
````

## Os principais conceitos legandados foram:

- dotenv -> lê um arquivo .env com configurações sensíveis (ex: senha do banco)

- env::var -> lê variáveis de ambiente

- Result<()> -> o main agora pode retornar erro

- PgPoolOptions / Pool -> gerencia múltiplas conexões com o banco de dados

- move → transfere posse de variáveis para dentro da closure

- app_data → compartilha dados (como o pool) entre todas as rotas

- .clone() → cria uma cópia do pool para cada thread