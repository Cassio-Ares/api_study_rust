# Inicio do projeto

- Dockerfile

```
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/api_study_rust .

CMD ["./api_study_rust"]
```

## Rodar cargo init sem o rust na maquina

- docker run  --rm -v $(pwd):/app -w /app rust:latest cargo init

## Rodar dockerfile
- docker build -t api_study_rust -f docker/Dockerfile .

### Cargo.toml inicial
````
[package]
name = "payment_api"
version = "0.1.0"
edition = "2024"

[dependencies]
actix-web = "4
````


## iniciando a main.rs

````
// Importa os componentes necessários do framework Actix-web:
// ´get´ -> macro para definir rota HTTP GET
// ´App´ -> estrutura principal da aplicação web
// ´HttpServer´ -> servidor HTTP
// ´Responder´ -> trait para tipos que podem ser retornados como resposta HTTP 
use actix_web::{get, App, HttpServer, Responder};


// Macro que define esta função como uma rota HTTP GET no caminho "/hello"
// Quando alguém acessar GET http://127.0.0.1:8080/hello, esta função será chamada
#[get("/hello")]
// Função assíncrona (não bloqueia a thread enquanto espera)
// `impl Responder` = retorna qualquer tipo que saiba virar uma resposta HTTP
async fn hello() -> impl Responder{
   "Hello"
}


// Macro que define o ponto de entrada da aplicação com suporte assíncrono do Actix
#[actix_web::main]
// Função principal assíncrona — aqui o servidor é configurado e iniciado
async fn main(){
  // Cria um novo servidor HTTP
  // O `||` é uma closure (função anônima) que constrói a aplicação para cada thread
  HttpServer::new(|| {
    // Cria a aplicação e registra a rota `hello` como um serviço
    App::new()
       .service(hello)   
  })
  // Liga o servidor ao endereço IP local (127.0.0.1) na porta 8080
  // O `?` propaga o erro caso a porta já esteja em uso
  .bind(("127.0.0.1", 8080))?
  // Inicia o servidor e começa a aceitar requisições
  .run()
  // `await` aguarda o servidor rodar indefinidamente (até ser encerrado)
  .await
}

````

## Os principais conceitos legendados aqui foram:

- use -> importação de dependências

- #[get(...)] -> macro que mapeia a função a uma rota HTTP GET

- async fn -> função assíncrona (não bloqueia a thread)

- impl Responder -> tipo de retorno genérico para respostas HTTP

- || clorure -> função anônima usada para construir app

- .service() -> registro de rota

- .bind() -> endereço e porta do servidor

- ? -> propagação de erro

- .await ->  aguarda a execução assíncrona terminar


## Macro 

* O que é uma Macro?
  Uma macro é um "atalho de código" — ela gera código automaticamente para você nos bastidores, antes do programa ser compilado.

  Pense assim:
  🧙 Você escreve uma linha → a macro expande para várias linhas de código complexo automaticamente.

````
// Você escreve isso:
#[get("/hello")]
async fn hello() -> impl Responder { ... }

// A macro gera por baixo algo como:
// - Registra a rota "/hello"
// - Associa ao método HTTP GET
// - Conecta à função hello()
// (são dezenas de linhas que você não precisa escrever)

// Você escreve isso:
#[actix_web::main]
async fn main() { ... }

// A macro transforma o async fn main() em um
// runtime assíncrono completo, configurando
// todas as threads e o event loop por baixo dos panos
````

 
 ## Thread

 * O que é uma Thread?
 Uma thread é uma linha de execução dentro do seu programa — ou seja, uma tarefa rodando de forma independente.

EX:
 🍽️ Com 1 garçom (1 thread) → atende uma mesa por vez, as outras esperam
 🍽️ Com vários garçons (várias threads) → várias mesas atendidas ao mesmo tempo

````
HttpServer::new(|| {        // <- esta closure roda UMA VEZ por thread
    App::new()
        .service(hello)
})
````

O Actix cria **várias threads automaticamente** (geralmente uma por núcleo do CPU), e para **cada thread** ele chama esse `||` para montar uma cópia da aplicação.

CPU tem 4 núcleos → Actix cria 4 threads

Thread 1 → atende requisição do usuário A

Thread 2 → atende requisição do usuário B

Thread 3 → atende requisição do usuário C

Thread 4 → atende requisição do usuário D
             (tudo ao mesmo tempo!)

