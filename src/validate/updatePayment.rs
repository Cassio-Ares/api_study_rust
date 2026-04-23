use validator::{Validate, ValidationError};
use crate::models::payment_model::{UpdatePayment, PaymentMethod };

impl Validate for UpdatePayment{
  fn validate(&self)-> Result<(), validator::ValidationError>{
    let mut errors = validator::ValidationErrors::new();

    if self.payment_method != PaymentMethod::CreditCard && self.payment_method != PaymentMethod::DebitCard && self.payment_method != PaymentMethod::Money && self.payment_method != PaymentMethod::Pix {
      errors.add("payment_method", ValidationError::new("Invalid payment method"));
    }

    if self.cuurrency.is_some() && self.currency.as_ref().unwrap().trim().is_empty() {
      errors.add("currency", ValidationError::new("Currency cannot be empty"));
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }
}