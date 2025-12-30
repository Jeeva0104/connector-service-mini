use serde::{Deserialize, Serialize};
use strum::Display;
#[derive(Debug, Serialize, Deserialize, Display)]
pub enum PaymentStatus {
    #[strum(serialize = "pending")]
    Pending,
}
