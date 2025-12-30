use actix_web::http::header::HeaderValue;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Application Response enum with all variants
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApplicationResponse<T> {
    Json(T),
    StatusOk,
    TextPlain(String),
    FileData((Vec<u8>, String)),
    JsonForRedirection(T),
    Form(RedirectionData),
}

#[derive(Debug, Serialize)]
pub struct RedirectionData {
    pub redirect_form: String,
    pub payment_method_data: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
}

// Session State
#[derive(Debug, Clone)]
pub struct SessionState {
    pub user_id: Option<String>,
    pub session_id: String,
}

// Request State
#[derive(Debug, Clone)]
pub struct ReqState {
    pub request_id: String,
    pub flow_name: String,
}

// API Event Metric trait
pub trait ApiEventMetric {
    fn event_type(&self) -> &'static str;
}

// Flow Metric trait
pub trait FlowMetric {
    fn flow_name(&self) -> &str;
}

// Vehicle Flow
pub struct VehicleFlow;

impl FlowMetric for VehicleFlow {
    fn flow_name(&self) -> &str {
        "vehicle_management"
    }
}

pub struct AuthorizeFlow;

impl FlowMetric for AuthorizeFlow {
    fn flow_name(&self) -> &str {
        "authorize"
    }
}

// Configuration for header masking
#[derive(Debug, Clone)]
pub struct HeaderMaskingConfig {
    pub keys: Vec<String>,
}

impl Default for HeaderMaskingConfig {
    fn default() -> Self {
        Self {
            keys: vec!["authorization".to_string(), "x-api-key".to_string()],
        }
    }
}

// Tag enum for logging
#[derive(Debug)]
pub enum Tag {
    BeginRequest,
    EndRequest,
}

// Vehicle Error
#[derive(Debug)]
pub struct VehicleError {
    pub message: String,
}

impl std::fmt::Display for VehicleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VehicleError {}
