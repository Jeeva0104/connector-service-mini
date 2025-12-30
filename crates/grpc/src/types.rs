use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardDetails {
    pub card_number: i64,
    pub card_cvc: i64,
    pub card_issuer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Card(CardDetails),
}
// Authorize request type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentServiceAuthorizeRequest {
    pub amount: i64,
    pub minor_amount: i64,
    pub payment_method: PaymentMethod,
}
