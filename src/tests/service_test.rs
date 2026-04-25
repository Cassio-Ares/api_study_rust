use crate::common::setup_test_db; 
use crate::service::payment_service;
use crate::model::payment::{NewPayment, Payment, PaymentMethod, PaymentStatus};
use uuid::Uuid;
use validator::Validate;

#[tokio::test]
async fn test_create_payment() {
  let pool = setup_test_db().await;

  let new_payment = NewPayment {
    amount: 100.0,
    currency: "USD".to_string(),
    payment_method: PaymentMethod::CreditCard,
    payment_reason: "Test Payment".to_string(),
    status: PaymentStatus::Completed,
  };

  let payment  = payment_service::create_new_payment_service(&pool, new_payment).await.unwrap();

  assert_eq!(payment.amount, 100.0);
  assert_eq!(payment.currency, "USD");
  assert_eq!(payment.payment_method, PaymentMethod::CreditCard);
  assert_eq!(payment.payment_reason, "Test Payment");
  assert_eq!(payment.status, PaymentStatus::Completed);
}

#[tokio::test]
async fn test_create_pending_payment() {
    let pool = setup_test_db().await;
    let new_payment = NewPayment {
        amount: 50.0,
        currency: "USD".to_string(),
        payment_method: PaymentMethod::CreditCard,
        payment_reason: "Test Pending Payment".to_string(),
        status: PaymentStatus::Pending,
    };

    let payment = payment_service::create_new_payment_service(&pool, new_payment).await.unwrap();

    assert_eq!(payment.amount, 50.0);
    assert_eq!(payment.payment_reason, "Test Pending Payment");
    assert_eq!(payment.status, PaymentStatus::Pending);
}


#[tokio::test]
async fn test_get_all_payments() {
    let pool = setup_test_db().await;
   
    let new_payment = NewPayment {
        amount: 100.0,
        currency: "USD".to_string(),
        payment_method: PaymentMethod::CreditCard,
        payment_reason: "Test Payment".to_string(),
        status: PaymentStatus::Completed,
    };

    for _ in 0..2 {
        payment_service::create_new_payment_service(&pool, new_payment.clone()).await.unwrap();
    }
    
    let payments = payment_service::get_all_payment_service(&pool).await.unwrap();

    assert!(!payments.is_empty());
    assert_eq!(payments.len(), 2);
}


#[tokio::test]
async fn test_get_payment_by_uuid() {
  let pool = setup_test_db().await;

  let new_payment = NewPayment {
    amount: 100.0,
    currency: "USD".to_string(),
    payment_method: PaymentMethod::CreditCard,
    payment_reason: "Test Payment".to_string(),
    status: PaymentStatus::Completed,
  };

  let payment = payment_service::create_new_payment_service(&pool, new_payment).await.unwrap();

  let fetched_payment = payment_service::get_payment_by_uuid_service(&pool, payment.uuid).await.unwrap();


 let fetched = fetched_payment.unwrap();
 assert_eq!(fetched.uuid, payment.uuid);
 assert_eq!(fetched.amount, payment.amount);
}


#[tokio::test]
async fn test_refund_payment_service_exceed_amount() {
    let pool = setup_test_db().await;

    let new_payment = NewPayment {
        amount: 100.0,
        currency: "USD".to_string(),
        payment_method: PaymentMethod::CreditCard,
        payment_reason: "Test Payment".to_string(),
        status: PaymentStatus::Completed,
    };

    let created_payment = payment_service::create_new_payment_service(&pool, new_payment).await.unwrap();

    let result = payment_service::refund_payment_service(&pool, created_payment.uuid, 150.0).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Amount to refund is greater than the original payment amount.");
}