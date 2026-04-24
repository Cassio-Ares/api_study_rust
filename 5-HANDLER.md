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


pub async fn get_payment_by_uuid(
  pool: web::Data<PgPool>, 
  uuid: web::Path<Uuid> // extrai o Uuid da URL /payment/{uuid}
) -> impl Responder {
  // Extrai o Uuid de dentro do web::Path
  let payment_uuid = path.into_inner()

 // agora o match tem 3 (ok, none, error) opções e não 2 
  match payment_service::get_payment_by_uuid_service(
    &pool,
    payment_uuid
  ).await {
    Ok(Some(payment))=> HttpResponse::ok().json(payment),
    // ⚠️ Caso o serviço retorne Ok, mas não encontre nada (None) → 404 Not Found
    Ok(None) => HttpResponse::NotFound().body("Payment not found"),
    Err(e) => HttpResponse::InternalServerError().body(e.to_string())
  }
}

pub async fn update_payment_handler(
  pool: web::Data<PgPool>,
  path: web::Path<Uuid>, // Uuid do payment na URL
  update_data: web::Json<UpdatePayment>, // novo status no corpo da requisição
) -> impl Responder {
  let uuid = path.into_inner();

  match payment_service::update_payment_service(&pool, uuid, update_data.into_inner()).await {
    Ok(Some(payment))=> HttpResponse::Ok()Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
  }
}

pub async fn refund_payment_handler(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    amount: web::Json<f64> // valor a reembolsar no corpo da requisição
) -> impl Responder {

    let payment_id = path.into_inner();
    let amount_to_refund = amount.into_inner();

  ...    
}
 ````


## Validator no service e handler qual a diferença 

- No projeto atual usamos o Validator no service 

com: new_payment.validate()?;
````
// Fluxo:
Handler recebe JSON
      ↓
Service → validate()? ← falhou?
      ↓                    ↓
   continua            erro sobe via `?`
      ↓                    ↓
Repository            Handler captura no match
                           ↓
                      Err(e) → HttpResponse::BadRequest()

// ✅ HANDLER — captura e transforma em HTTP
pub async fn create_payment_handler(
    pool: web::Data<PgPool>,
    new_payment: web::Json<NewPayment>
) -> impl Responder {
    match payment_service::create_new_payment_service(
        &pool,
        new_payment.into_inner()
    ).await {
        Ok(payment) => HttpResponse::Created().json(payment),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        //         ↑ erro de validação cai aqui
        //           com a mensagem correta do validator
    }
}

````

- Agora validator no handler 

Diferente do validator no service que usamos "?" e subimos o retorno do erro no handler já tratamos o erro aqui mesmo. 

````
pub async fn create_payment_handler(
    pool: web::Data<PgPool>,
    new_payment: web::Json<NewPayment> // 1. extrai
) -> impl Responder {
    if let Err(errors) = new_payment.validate() { // 2. valida manualmente
        return HttpResponse::BadRequest().json(errors);
    }
    // ...
} 
````

### Opção com o validade de dados antes de entrar no handler

- primeiro precisamos colocar "actix-web-validator = "5.0"" no cargo.toml

fluxo: 
````
Cliente envia JSON inválido { amount: -50 }
              ↓
     actix-web-validator
     chama .validate() automaticamente
              ↓
         falhou?
         ↓          ↓
        sim         não
         ↓          ↓
    400 BadRequest  entra na função
    automático ✅   com dado válido ✅
````


Para montar o codigo agora ao inves de usarmos do Json do web::Json do actix_web vamos usar o Json do use actix_web_validator::Json; 

OBS: como a validações é antes de qualquer manipulação o uso de default não funciona.

````
use actix_web_validator::Json; // Importamos o Json do validador, não do actix-web puro

pub async fn create_payment_handler(
    pool: web::Data<PgPool>, 
    // MÁGICA: Se o JSON enviado for inválido (ex: valor negativo), 
    // o Actix nem executa essa função. Ele já retorna 400 automaticamente!
    new_payment: Json<NewPayment> 
) -> impl Responder {
    // Aqui você já tem a certeza absoluta que o dado é válido!
    match payment_service::create_new_payment(&pool, new_payment.into_inner()).await {
        Ok(payment) => HttpResponse::Created().json(payment),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
````