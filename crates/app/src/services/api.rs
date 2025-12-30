use actix_web::{web, HttpResponse, HttpRequest, http::header::HeaderValue, http::header::HeaderMap};
use std::future::Future;
use std::collections::HashMap;
use std::time::Instant;
use serde::Serialize;
use std::fmt::Debug;
use error_stack::Result as ErrorStackResult;
use crate::types::{ApplicationResponse, SessionState, ReqState, Tag, FlowMetric, ApiEventMetric};
use crate::state::AppState;
use crate::http_utils::*;

pub type CustomResult<T, E> = ErrorStackResult<T, E>;

// Logger module
pub mod logger {
    pub fn info<T: std::fmt::Debug>(msg: T) {
        tracing::info!("{:?}", msg);
    }
}

// Updated server_wrap_util to auto-wrap responses
pub async fn server_wrap_util<T, Q, F, Fut, E>(
    flow: &impl FlowMetric,
    state: web::Data<AppState>,
    _incoming_request_header: &HeaderMap,
    request: &HttpRequest,
    payload: T,
    func: F,
) -> CustomResult<ApplicationResponse<Q>, E>
where
    F: Fn(SessionState, SessionState, T, ReqState) -> Fut,
    Fut: Future<Output = CustomResult<Q, E>>,  // Note: Returns Q directly, not ApplicationResponse<Q>
    Q: Serialize + Debug + ApiEventMetric,
    T: Debug + Serialize + ApiEventMetric,
    E: std::error::Error + Send + Sync + 'static,
{
    // Create simplified session state (no auth)
    let session_state = SessionState {
        user_id: Some("default_user".to_string()),
        session_id: uuid::Uuid::new_v4().to_string(),
    };
    
    // Create request state
    let req_state = ReqState {
        request_id: uuid::Uuid::new_v4().to_string(),
        flow_name: flow.flow_name().to_string(),
    };
    
    tracing::info!(
        "Processing request: {} for flow: {}", 
        req_state.request_id, 
        req_state.flow_name
    );
    
    // Execute business logic and auto-wrap in ApplicationResponse::Json
    match func(session_state.clone(), session_state, payload, req_state).await {
        Ok(data) => Ok(ApplicationResponse::Json(data)),
        Err(err) => Err(err),
    }
}

// Updated server_wrap function with auto-wrapping
pub async fn server_wrap<'a, T, Q, F, Fut, E>(
    flow: impl FlowMetric,
    state: web::Data<AppState>,
    request: &'a HttpRequest,
    payload: T,
    func: F,
) -> HttpResponse
where
    F: Fn(SessionState, SessionState, T, ReqState) -> Fut,
    Fut: Future<Output = CustomResult<Q, E>>,  // Note: Returns Q directly, not ApplicationResponse<Q>
    Q: Serialize + Debug + ApiEventMetric + 'a,
    T: Debug + Serialize + ApiEventMetric,
    E: std::error::Error + Send + Sync + 'static,
{
    let request_method = request.method().as_str();
    let url_path = request.path();

    let unmasked_incoming_header_keys = &state.conf().unmasked_headers.keys;

    let incoming_request_header = request.headers();

    let incoming_header_to_log: HashMap<String, HeaderValue> =
        incoming_request_header
            .iter()
            .fold(HashMap::new(), |mut acc, (key, value)| {
                let key = key.to_string();
                if unmasked_incoming_header_keys.contains(&key.as_str().to_lowercase()) {
                    acc.insert(key.clone(), value.clone());
                } else {
                    acc.insert(key.clone(), HeaderValue::from_static("**MASKED**"));
                }
                acc
            });

    tracing::Span::current().record("request_method", request_method);
    tracing::Span::current().record("request_url_path", url_path);

    let _start_instant = Instant::now();

    logger::info(format!(
        "tag = {:?}, payload = {:?}, headers = {:?}",
        Tag::BeginRequest, payload, incoming_header_to_log
    ));

    let server_wrap_util_res = server_wrap_util(
        &flow,
        state.clone(),
        incoming_request_header,
        request,
        payload,
        func,
    )
    .await
    .map(|response| {
        logger::info(format!("api_response = {:?}", response));
        response
    });

    let res = match server_wrap_util_res {
        Ok(ApplicationResponse::Json(response)) => match serde_json::to_string(&response) {
            Ok(res) => http_response_json(res),
            Err(_) => http_response_err(
                r#"{
                    "error": {
                        "message": "Error serializing response from connector"
                    }
                }"#,
            ),
        },
        Ok(ApplicationResponse::StatusOk) => http_response_ok(),
        Ok(ApplicationResponse::TextPlain(text)) => http_response_plaintext(text),
        Ok(ApplicationResponse::FileData((file_data, content_type))) => {
            http_response_file_data(file_data, content_type)
        }
        Ok(ApplicationResponse::JsonForRedirection(response)) => {
            match serde_json::to_string(&response) {
                Ok(res) => http_redirect_response(res, response),
                Err(_) => http_response_err(
                    r#"{
                    "error": {
                        "message": "Error serializing response from connector"
                    }
                }"#,
                ),
            }
        }
        Ok(ApplicationResponse::Form(redirection_data)) => {
            let config = state.conf();
            build_redirection_form(
                &redirection_data.redirect_form,
                redirection_data.payment_method_data,
                redirection_data.amount,
                redirection_data.currency,
                config,
            )
        }
        Err(err) => {
            tracing::error!("Request failed: {:?}", err);
            http_response_err(
                r#"{
                    "error": {
                        "message": "Internal server error"
                    }
                }"#,
            )
        }
    };

    res
}
