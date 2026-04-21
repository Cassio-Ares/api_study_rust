use validator::{Validate, ValidationError};
use crate::models::payment_model::{UpdatePaymentStatus, PaymentStatus };

impl Validate for UpdatePaymentStatus {
  fn validate(&self)-> Result<(), validator::ValidationError>{
    let mut errors = validator::ValidationErrors::new();

    if self.status != PaymentStatus::Pending && self.status != PaymentStatus::Completed && self.status != PaymentStatus::Failed {
      errors.add("status", ValidationError::new("Invalid payment status"));
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }
}