# Handler

````
// Importe de ferramentas do Actix_web
// web -> extrator de dados(JSON, PATH, DATA, ...)
// HttpResponse -> construtor de resosta HTTP
// Responder -> trait que qualquer retorno de handler deve implementar

use actix_web::{web, HttpResponse, Responder};

// Pool de conexões do SQLx para injetarmos o banco nos handlers
use sqlx::PgPool;

// Tipo UUID para lidar com os identificadores únicos dos pagamentos
use uuid::Uuid;

use crate::models::payment::NewPayment;
use crate::services::payment_service;

pub async fn create_payment_handler(
  pool: web::Data<PgPool>,  // pool injetado pelo Actix automaticamente
  new_payment: web::Json<NewPayment> // Extrai o JSON do corpo da requisição automaticamente deserializado para NewPayment
) -> impl Responder  // Retorna algo que o Actix saiba transformar em resposta HTTP
{

  // `match` no Result do service — trata Ok e Err separadamente
  match payment_service::create_new_payment_service(
    &pool, 
    //  O .into_inner() transforma o web::Json<NewPayment> na struct NewPayment pura.
    new_payment.into_inner() // extrai o NewPayment de dentro do web::Json
  ).await(
    // ✅ se de ok, HttpResponse constroi a responta da forma que HTTP aceita
    // status que vem de Created = 201 e os dados como json
    Ok(payment) => HttpResponse::Created().json(payment),

    // ❌ se der erro  HttpResponse cria um erro no formato que HTTP aceita 
    // status InternalServerError  e a resposta no body e com formato string (e.to_string())
    Err(e) => HttpResponse::InternalServerError().body(
      e.to_string(),
    )  
  )
}

pub async fn get_all_payments_handler(
    pool: web::Data<PgPool>
) -> impl Responder {

    match payment_service::get_all_payments_service(&pool).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
 ````