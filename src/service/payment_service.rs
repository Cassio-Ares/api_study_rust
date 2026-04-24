sqlx::PgPool,
use uuid::Uuid;
use validator::Validate;

use crate::repositories::payment_repository;
use crate::models::payment_model::{NewPayment, Payment, UpdatePayment, UpdatePaymentStatus};

pub async fn create_new_payment_service(
  pool: &PgPool,
  new_payment: NewPayment,
) -> Result<Payment, Box<dyn std::error::Error>> {
  new_payment.validate()?;

  let payment = payment_repository::create_payment(pool, new_payment).await?;

  Ok(payment)
}

pub async fn get_all_payments_service(
  pool: &PgPool,
) -> Result<Vec<Payment>, Box<dyn std::error::Error>> {
  let payments = payment_repository::get_all_payments(pool).await?;

  Ok(payments)
}

pub async fn get_payment_by_uuid_service(
  pool: &PgPool,
  payment_uuid: Uuid,
)-> Result<Option<Payment>, Box<dyn std::error::Error>>{
  let payment = payment_repository::get_payment_by_uuid(pool, payment_uuid).await?;
  Ok(payment)
}

pub async fn update_payment_service(pool: &PgPool, uuid: Uuid, update_data: UpdatePayment) -> Result<Option<Payment>, Box<dyn std::error::Error>> {
    update_data.validate()?;
    let payment = payment_repository::update_payment(pool, uuid, update_data).await?;
    Ok(payment)
}

pub async fn update_payment_status_service(pool: &PgPool, uuid: Uuid, update_data: UpdatePaymentStatus) -> Result<Option<Payment>, Box<dyn std::error::Error>> {
    update_data.validate()?;
    let payment = payment_repository::update_payment_status(pool, uuid, update_data).await?;
    Ok(payment)
}


pub async fn refund_payment_service(pool: &PgPool, uuid: Uuid, amount_to_refund: f64) -> Result<Option<Payment>, Box<dyn std::error::Error>> {
    let existing_payment = payment_repository::get_payment_by_uuid(pool, uuid).await?;

    if let Some(mut payment) = existing_payment {
        if payment.amount < amount_to_refund {
            return Err("Amount to refund is greater than the original payment amount.".into());
        }
        payment.amount -= amount_to_refund;
        payment.status = if payment.amount == 0.0 { "refunded".to_string() } else { "partially_refunded".to_string() };
        let updated_payment = payment_repository::update_payment_for_refund(pool, uuid, payment.amount, payment.status).await?;
        Ok(updated_payment)
    } else {
        Ok(None)
    }
}