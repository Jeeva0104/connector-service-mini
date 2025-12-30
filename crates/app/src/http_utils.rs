use actix_web::{HttpResponse, http::header::ContentType};

pub fn http_response_json(json_str: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_str)
}

pub fn http_response_err(error_json: &str) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type(ContentType::json())
        .body(error_json.to_string())
}

pub fn http_response_ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn http_response_plaintext(text: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(text)
}

pub fn http_response_file_data(file_data: Vec<u8>, content_type: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(content_type)
        .body(file_data)
}

pub fn http_redirect_response<T: serde::Serialize>(json_str: String, _response: T) -> HttpResponse {
    HttpResponse::Found()
        .content_type(ContentType::json())
        .body(json_str)
}

pub fn build_redirection_form(
    _redirect_form: &str,
    _payment_method_data: Option<String>,
    _amount: Option<f64>,
    _currency: Option<String>,
    _config: &crate::state::AppConfig,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<html><body>Redirection Form</body></html>")
}
