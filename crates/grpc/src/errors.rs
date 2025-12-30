#[derive(Debug, Clone)]
pub struct PaymentAuthorizationError {
    pub status: String,
    pub error_message: Option<String>,
    pub error_code: Option<String>,
    pub status_code: Option<u32>,
}

impl PaymentAuthorizationError {
    pub fn new(
        status: String,
        error_message: Option<String>,
        error_code: Option<String>,
        status_code: Option<u32>,
    ) -> Self {
        Self {
            status,
            error_message,
            error_code,
            status_code,
        }
    }
}
