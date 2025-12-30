use crate::payment::{Payment, PaymentOperationInternal, PaymentService};
use error_stack::Result as ErrorStackResult;
use grpc::types::PaymentServiceAuthorizeRequest;

// Error types for vehicle operations
#[derive(Debug)]
pub struct PaymentServiceAuthorizeError {
    pub message: String,
}

impl std::fmt::Display for PaymentServiceAuthorizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PaymentServiceAuthorizeError {}
pub type CustomResult<T, E> = ErrorStackResult<T, E>;

pub type PaymentAuthrorizeResponse = String;

pub async fn payment_authorize(
    payload: PaymentServiceAuthorizeRequest,
) -> CustomResult<PaymentAuthrorizeResponse, PaymentServiceAuthorizeError> {
    let _ = Payment.authorize(payload).await;
    Ok("success".to_string())
}
