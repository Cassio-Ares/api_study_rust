use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::payment_model::{NewPayment, UpdatePayment, UpdatePaymentStatus, Payment};
use crate::services::payment_service;

pub async fn create_payment_handler(
  pool: web::Data<PgPool>,
  new_payment: web::Json<NewPayment>,
) -> impl Responder {
  match payment_service::create_new_payment_service(
    &pool,
    new_payment.into_inner(),
  ).await
  {
    Ok(payment) => HttpResponse::Created().json(payment),
    Err(e) => HttpResponse::BadRequest().body(e.to_string()),
  }
}

pub async fn get_all_payments_handler(
  pool: web::Data<PgPool>,) -> impl Responder {
    
    match payment_service::get_all_payment_service(&pool).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_payment_by_uuid_handler(
  pool: web::Data<PgPool>,
  path: web::Path<Uuid>,
)-> impl Responser{
  let payment_uuid = path.into_inner();

  match payment_service::get_payment_by_uuid_service(&pool, payment_uuid).await{
    Ok(Some(payment)) => HttpResponse::Ok().json(payment),
    Ok(None) => HttpResponse::NotFound().body("Payment not found"),
    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
  }
}



pub async fn update_payment_handler(
  pool: web::Data<PgPool>,
  path: web::Path<Uuid>,
  update_data: web::Json<UpdatePayment>,
) -> impl Responder {
  let uuid = path.into_inner();

  match payment_service::update_payment_service(&pool, uuid, update_data.into_inner()).await {
    Ok(Some(payment))=> HttpResponse::Ok()Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().body("Payment not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
  }
}


pub async fn update_payment_status_handler(
  pool: web::Data<PgPool>,
  path: web::Path<Uuid>,
  update_data: web::Json<UpdatePaymentStatus>,
) -> impl Responder {
  let uuid = path.into_inner();

  match payment_service::update_payment_status_service(
    &pool,
    uuid,
    update_data.into_inner()
  ).await {
    Ok(Some(payment))=> HttpResponse::Ok().json(payment),
    Ok(None)=> HttpResponse::NotFound().body("Payment not found"),
    Err(e)=> HttpResponse::InternalServerError().body(e.to_string()),
  }
}


pub async fn refund_payment_handler(
  pool: web::Data<PgPool>,
  path: Path<Uuid>,
  amount_to_refund: web::Json<f64>,
)-> impl Responder{
  let uuid = path.into_inner();
  let amount = amount_to_refund.into_inner();

  match payment_service::refund_payment_service(
    &pool, 
    uuid,
    amount
  ).await{
    Ok(Some(payment))=> HttpResponse::Ok().json(payment),
    Ok(None)=> HttpResponse::NotFound().body("Payment not found"),
    Err(e)=> HttpResponse::InternalServerError().body(e.to_string()),
  }
}