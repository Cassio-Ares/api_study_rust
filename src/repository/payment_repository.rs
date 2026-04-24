use sqlx::{PgPool, Postgres, query_as};
use uuid::Uuid;
use chrono::Utc;

use crate::models::payment_model::{Payment, NewPayment, PaymentMethod, PaymentStatus, UpdatePayment, UpdatePaymentStatus};

pub async fn create_payment(
    pool: &PgPool,
    new_payment: NewPayment,
) -> Result<Payment, sqlx::Error> {
  let payment = query_as::<Postgres, Payment>(
    "INSERT INTO payments (
        uuid, amount, currency, payment_method, payment_reason, status, created_at, updated_at
      ) VALUES (
         $1, $2, $3, $4, $5, $6, $7, $8
      ) RETURNING *",
  )
  .bind(Uuid::new_v4())
  .bind(new_payment.amount)
  .bind(new_payment.currency)
  .bind(new_payment.payment_method as PaymentMethod)
  .bind(new_payment.payment_reason)
  .bind(new_payment.status as PaymentStatus)
  .bind(Utc::now().naive_utc())
  .bind(Utc::now().naive_utc())
  .fetch_one(pool)
  .await?;

  Ok(payment)
}

pub async fn get_all_payments(pool: &PgPool) -> Result<Vec<Payment>, sqlx::Error> {
  let payments = query_as::<Postgres, Payment>(
    "SELECT * FROM payments",
  )
  .fetch_all(pool)
  .await?;

  Ok(payments)
}

pub async fn get_payment_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Payment, sqlx::Error> {
  let payment = query_as::<Postgres, Payment>(
    "SELECT * FROM payments WHERE uuid = $1",
  )
  .bind(uuid)
  .fetch_one(pool)
  .await?;

  Ok(payment)
}


pub async fn update_payment(pool: &PgPool, uuid:Uuid, update_data: UpdatePayment)-> Result<Payment, sqlx::Error>{
  let payment = query_as::<Postgres, Payment>(
    "UPDATE payments SET
       payment_method = $1, currency = $2, status = $3, updated_at = $4 WHERE uuid = $5 RETURNING *"
  )
  .bind(update_data.payment_method as PaymentMethod)
  .bind(update_data.currency)
  .bind(update_data.status as PaymentStatus)
  .bind(Utc::now().naive_utc())
  .bind(uuid)
  .fetch_one(pool)
  .await?;

  Ok(payment)
}


pub async fn update_payment_status(pool: &PgPool, uuid: Uuid, new_status: UpdatePaymentStatus) -> Result<Payment, sqlx::Error> {
  let payment = query_as::<Postgres, Payment>(
    "UPDATE payments SET status = $1, updated_at = $2 WHERE uuid = $3 RETURNING *"
  )
  .bind(new_status.status as PaymentStatus)
  .bind(Utc::now().naive_utc())
  .bind(uuid)
  .fetch_one(pool)
  .await?;

  Ok(payment)
}


pub async fn update_payment_for_refund(pool: &PgPool, uuid: Uuid, new_amount: f64, status: String) -> Result<Option<Payment>, sqlx::Error> {
    let payment = query_as::<Postgres, Payment>(
        "UPDATE payments SET amount = $1, status = $2, updated_at = $3 WHERE uuid = $4 RETURNING *"
    )
    .bind(new_amount)
    .bind(status)
    .bind(Utc::now().naive_utc())
    .bind(uuid)
    .fetch_optional(pool)
    .await?;

    Ok(payment)
}