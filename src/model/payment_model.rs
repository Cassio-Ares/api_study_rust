use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    Money,
    Pix,
}

#[derive(Debug, Serialize, Deserialize, FromRow,)]
pub struct Payment {
 pub id: u32,
 pub uuid: Uuid,
 pub amount: f64,
 pub currency: String,
 pub payment_method: PaymentMethod,
 pub payment_reason: String,
 pub status: PaymentStatus,
 pub created_at: NaiveDateTime,
 pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct NewPayment {
    pub amount: f64,
    pub currency: Option<String>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_reason: String,
    pub status: PaymentStatus,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct UpdatePayment {
    pub payment_method: PaymentMethod,
    pub currency: String,
    pub status: PaymentStatus,
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]    
pub struct UpdatePaymentStatus {
    pub status: PaymentStatus,
}