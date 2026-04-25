use crate::model::payment_model::{NewPayment, UpdatePayment, UpdatePaymentStatus, PaymentStatus, PaymentMethod};

use validator::Validate;

#[test]
fn test_amount_zero_invalid(){
    let payment = NewPayment {
        amount: 0.0,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };

    assert!(payment.validate().is_err());
}

#[test]
fn test_amount_negative_invalid(){
    let payment = NewPayment {
        amount: -10.0,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };

    assert!(payment.validate().is_err());
}

#[test]
fn test_amount_valido() {
    let payment = NewPayment {
        amount: 0.01,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };
    assert!(payment.validate().is_ok());
}

#[test]
fn test_currency_empty(){
    let payment = NewPayment {
        amount: 10.0,
        currency: "".to_string(),
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };

   let result = payment.validate();
   assert!(result.is_err());

   let errors = result.unwrap_err();
    // Verifica se o erro é especificamente no campo "currency"
    assert!(errors.field_errors().contains_key("currency"));
}

#[test]
fn test_currency_spaces_invalid() {
    let payment = NewPayment {
        amount: 100.0,
        currency: "   ".to_string(), // ← só espaços — inválido
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };
    assert!(payment.validate().is_err());
}

#[test]
fn test_multiple_invalid_fields(){
    let payment = NewPayment {
        amount: -50.0, // ← inválido
        currency: "".to_string(), // ← inválido
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };

    let result = payment.validate();
    assert!(result.is_err());

    let errors = result.unwrap_err();
    let field_errors = errors.field_errors();

    // Verifica que AMBOS os campos têm erro
    assert!(field_errors.contains_key("amount"));
    assert!(field_errors.contains_key("currency"));
}

#[test]
fn test_message_erro_amount() {
    let payment = NewPayment {
        amount: 0.0,
        currency: "".to_string(), 
        description: "Test payment".to_string(),
        method: PaymentMethod::CreditCard,
        status: PaymentStatus::Completed,
    };

    let errors = payment.validate().unwrap_err();
    let amount_errors = &errors.field_errors()["amount"];

    // Verifica a MESSAGE do erro (a biblioteca validator não suporta codes customizados)
    assert_eq!(
        amount_errors[0].message.as_ref().unwrap().as_ref(),
        "Amount must be greater than zero"
    );
}