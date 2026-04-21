use validator::{Validate, ValidationError};
use crate::models::payment_model::{NewPayment, PaymentMethod, PaymentStatus};

impl Validate for NewPayment {
  fn validate(&self)-> Result<(), validator::ValidationErrors>{
    let mut errors = validator::ValidationErrors::new();

    if self.amount <= 0.0 {
      errors.add("amount", ValidationError::new("Amount must be greater than zero"));
    }

    if self.status == PaymentStatus::Completed && self.currency.as_ref().map_or(true, |c| c.trim().is_empty()){
      errors.add("currency", ValidationError::new("Currency cannot be empty"));
    }

    if self.status == PaymentStatus::Completed && (self.payment_method.is_none() || (self.payment_method != Some(PaymentMethod::CreditCard) && self.payment_method != Some(PaymentMethod::DebitCard) && self.payment_method != Some(PaymentMethod::Money) && self.payment_method != Some(PaymentMethod::Pix))) {
      errors.add("payment_method", ValidationError::new("Invalid payment method"));
    }

    if self.payment_reason.trim().is_empty() {
      errors.add("payment_reason", ValidationError::new("Payment reason cannot be empty"));
    }

    if self.status != PaymentStatus::Pending && self.status != PaymentStatus::Completed && self.status != PaymentStatus::Canceled {
      errors.add("status", ValidationError::new("Invalid payment status"));
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }
}