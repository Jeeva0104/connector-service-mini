use crate::services::api::server_wrap;
use crate::state::AppState;
use crate::types::{ApiEventMetric, AuthorizeFlow, ReqState, SessionState};
use actix_web::{web, HttpRequest, Responder, Scope};
use grpc::types::PaymentServiceAuthorizeRequest;
use payment::core::{payment_authorize, PaymentAuthrorizeResponse};

// Implement ApiEventMetric for the app crate types
impl ApiEventMetric for PaymentServiceAuthorizeRequest {
    fn event_type(&self) -> &'static str {
        "authorize_request"
    }
}

impl ApiEventMetric for PaymentAuthrorizeResponse {
    fn event_type(&self) -> &'static str {
        "authorize_response"
    }
}

pub async fn payment_authorize_request(
    state: web::Data<AppState>,
    request: HttpRequest,
    payload: web::Json<PaymentServiceAuthorizeRequest>,
) -> impl Responder {
    let flow = AuthorizeFlow;
    let app_state = state.get_ref().clone();
    // let _ = Payments::new();
    Box::pin(server_wrap(
        flow,
        state,
        &request,
        payload.into_inner(),
        move |_session_state: SessionState,
              _user_data: SessionState,
              request: PaymentServiceAuthorizeRequest,
              req_state: ReqState| {
            let app_state = app_state.clone();
            async move {
                println!(
                    "Request ID: {}, Flow: {}",
                    req_state.request_id, req_state.flow_name
                );

                // Use the new authorize function which returns CustomResult
                payment_authorize(request).await
            }
        },
    ))
    .await
}

pub struct Authorize;

impl Authorize {
    pub fn server(state: AppState) -> Scope {
        web::scope("/authorize")
            .app_data(web::Data::new(state))
            .service(
                web::scope("/v1")
                    .service(web::resource("").route(web::post().to(payment_authorize_request))),
            )
    }
}
