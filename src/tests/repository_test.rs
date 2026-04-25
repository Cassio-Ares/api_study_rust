use crate::common::setup_test_db; 
use crate::repository::payment_repository;
use crate::model::payment::{NewPayment, Payment, PaymentMethod, PaymentStatus};
use uuid::Uuid;
use validator::Validate;

#[tokio::test]
async fn test_create_payment() {
  let pool = setup_test_db().await;

  let new_payment = NewPayment {
    amount: 150.0,
    currency: "USD".to_string(),
    payment_method: PaymentMethod::CreditCard,
    payment_reason: "Test Payment".to_string(),
    status: PaymentStatus::Completed,
  };

  let payment  = payment_repository::create_payment(&pool, new_payment).await.unwrap();

  assert_eq!(payment.amount, 150.0);
  assert_eq!(payment.currency, "USD");
  assert_eq!(payment.payment_method, PaymentMethod::CreditCard);
  assert_eq!(payment.payment_reason, "Test Payment");
  assert_eq!(payment.status, PaymentStatus::Completed);
}


#[tokio::test]
async fn test_get_all_payments() {
  let pool = setup_test_db().await;

  let new_payment = NewPayment {
    amount: 150.0,
    currency: "USD".to_string(),
    payment_method: PaymentMethod::CreditCard,
    payment_reason: "Test Payment".to_string(),
    status: PaymentStatus::Completed,
  };

  for _ in 0..2 {
    payment_repository::create_payment(&pool, new_payment.clone()).await.unwrap();
  }

  let payments = payment_repository::get_all_payments(&pool).await.unwrap();

  assert!(!payments.is_empty());
  assert_eq!(payments.len(), 2);
}