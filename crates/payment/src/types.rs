// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct CardDetails {
//     pub card_number: i64,
//     pub card_cvv: i64,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "snake_case")]
// pub enum PaymentMethod {
//     Card(CardDetails),
// }
// // Authorize request type
// #[derive(Debug, Deserialize, Serialize)]
// pub struct PaymentServiceAuthorizeRequest {
//     pub amount: i64,
//     pub minor_amount: i64,
//     pub payment_method: PaymentMethod,
// }
