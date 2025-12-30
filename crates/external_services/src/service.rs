use common_enums::ApiClientError;
use common_utils::{
    errors::CustomResult,
    ext_traits::AsyncExt,
    request::{Method, Request, RequestContent},
};
use domain_types::{
    // connector_types::{ConnectorResponseHeaders, RawConnectorRequestResponse},
    errors::ConnectorError,
    router_data_v2::RouterDataV2,
    // router_response_types::Response,
    // types::Proxy,
    router_response_types::Response,
};
use error_stack::{report, ResultExt};
use hyperswitch_masking::{ErasedMaskSerialize, ExposeInterface, Maskable};
use interfaces::connector_integration_v2::BoxedConnectorIntegrationV2;
use reqwest::Client;
use serde_json::json;
use std::{collections::HashMap, str::FromStr, sync::RwLock, time::Duration};
// static DEFAULT_CLIENT: OnceCell<Client> = OnceCell::new();
fn extract_raw_connector_request(connector_request: &Request) -> String {
    // Extract actual body content
    let body_content = match connector_request.body.as_ref() {
        Some(request) => {
            match request {
                // For RawBytes (e.g., SOAP XML), use the string directly without JSON parsing
                RequestContent::RawBytes(_) => {
                    serde_json::Value::String(request.get_inner_value().expose())
                }
                // For other content types, try to parse as JSON
                RequestContent::Json(_)
                | RequestContent::FormUrlEncoded(_)
                | RequestContent::FormData(_)
                | RequestContent::Xml(_) => {
                    let exposed_value = request.get_inner_value().expose();
                    serde_json::from_str(&exposed_value).unwrap_or_else(|_| {
                        // tracing::warn!("failed to parse body as JSON, treating as string in extract_raw_connector_request");
                        serde_json::Value::String(exposed_value)
                    })
                }
            }
        }
        None => serde_json::Value::Null,
    };
    // Extract unmasked headers
    let headers_content = connector_request
        .headers
        .iter()
        .map(|(k, v)| {
            let value = match v {
                Maskable::Normal(val) => val.clone(),
                Maskable::Masked(val) => val.clone().expose().to_string(),
            };
            (k.clone(), value)
        })
        .collect::<HashMap<_, _>>();

    // Create complete request with actual content
    json!({
        "url": connector_request.url,
        "method": connector_request.method.to_string(),
        "headers": headers_content,
        "body": body_content
    })
    .to_string()
}

fn load_custom_ca_certificate_from_content(
    mut client_builder: reqwest::ClientBuilder,
    // cert_content: &str,
) -> CustomResult<reqwest::ClientBuilder, ApiClientError> {
    let cert = "-----BEGIN CERTIFICATE-----\nMIIDNTCCAh2gAwIBAgIULZTaHx0R3u0ua/wOA+7rHxSsABkwDQYJKoZIhvcNAQEL\nBQAwKDESMBAGA1UEAwwJbWl0bXByb3h5MRIwEAYDVQQKDAltaXRtcHJveHkwHhcN\nMjUxMDI4MDgxNDAyWhcNMzUxMDI4MDgxNDAyWjAoMRIwEAYDVQQDDAltaXRtcHJv\neHkxEjAQBgNVBAoMCW1pdG1wcm94eTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCC\nAQoCggEBANUNbRjA52/3aABvTEOWMiNRQZvrmHKul1ZwNeQ7mKIwMZstmahRc6xo\nTqv80yOurm+NGSc845zPCA2C6NGPJ182TF/JyjZ3svF+h5eA/IJ7EHSyeIE9k7k6\ntZ+q4TmWGrEY7O4AcZnynI2AzUeaVlehyM/P02hOx1pxPu0OCMDYAxRY86VnNII4\ncHYWhqQnxtBlsJxNwaLvYpiL+IOzolqWzatVMUjtSj54kMcB23MS2+GjwKJav8qm\nfzJe0uLY8FzcwpIdaIJ1TJ0inawxBA8FQlNrQNmwoh0Sk2HKLgGYduqJB23tMwdu\nZfz8rLPYDXcrM6DyCL3pmjUPJuyx3FECAwEAAaNXMFUwDwYDVR0TAQH/BAUwAwEB\n/zATBgNVHSUEDDAKBggrBgEFBQcDATAOBgNVHQ8BAf8EBAMCAQYwHQYDVR0OBBYE\nFBP4TdfnOBtlz/lr+A0pKSn1UChfMA0GCSqGSIb3DQEBCwUAA4IBAQBeZhahkWT5\n7HyLteOuY3CR9ZbYx+diEGwBwIeIH0ALt0b1DH+t5iLRARv1vLVv2EGJLuAFDDPy\n9/FgO9NOlJ9b0p/wo9aDx7rO9Vc+Xtr5Damr5+Re98mjzalIF7C34WAgE/0xmNxM\nXcVvhNcarkoJWxQ4asGmAq93KMiIt8F2/cHX4rikBXm3en0hWxy6oBi3lDzNbcZ/\n24Z9eh6szcyBCf6fHQiCtEHZij64A0PxHYKEwB+TFfvzkupvmA76C2MYyWzRkkn+\nRHsX+RzJx6qYXvUhdo93X9ciiCFI1Jdd5sKoLCMwz7Rnnr70dxFLSOZuZDQp4chQ\nQsn/SJD7GXtG\n-----END CERTIFICATE-----";
    let certificate = reqwest::Certificate::from_pem(cert.as_bytes())
        .change_context(ApiClientError::InvalidProxyConfiguration)
        .attach_printable("Failed to parse certificate PEM from provided content")?;
    client_builder = client_builder.add_root_certificate(certificate);
    Ok(client_builder)
}

fn get_client_builder(
    proxy_url: Option<&str>,
) -> CustomResult<reqwest::ClientBuilder, ApiClientError> {
    // let url = "http://localhost:8081".to_string();
    let mut client_builder = Client::builder().redirect(reqwest::redirect::Policy::none());

    if let Some(proxy_url) = proxy_url {
        // let t = proxy_url.to_string();
        client_builder = client_builder.proxy(
            reqwest::Proxy::https(proxy_url.to_string())
                .change_context(ApiClientError::InvalidProxyConfiguration)
                .inspect_err(|err| {
                    println!("{:?}", err);
                })?,
        );
    }
    client_builder = load_custom_ca_certificate_from_content(client_builder)?;
    Ok(client_builder)
}

fn get_base_client(proxy_url: Option<&str>) -> CustomResult<Client, ApiClientError> {
    // Use DEFAULT_CLIENT for non-proxy scenarios
    let client = {
        get_client_builder(proxy_url)?
            .build()
            .change_context(ApiClientError::ClientConstructionFailed)
            .attach_printable("Failed to construct default client")?
    };
    Ok(client)
}

pub fn create_client(proxy_url: Option<&str>) -> CustomResult<Client, ApiClientError> {
    get_base_client(proxy_url)
}

async fn handle_response(
    response: CustomResult<reqwest::Response, ApiClientError>,
) -> CustomResult<Result<Response, Response>, ApiClientError> {
    println!("{:?} RESPONSE", response);
    response
        .async_map(|resp| async {
            let status_code = resp.status().as_u16();
            println!("{:?} STATUS CODE", status_code);
            let headers = Some(resp.headers().to_owned());
            match status_code {
                200..=202 | 302 | 204 => {
                    let response = resp
                        .bytes()
                        .await
                        .change_context(ApiClientError::ResponseDecodingFailed)?;
                    Ok(Ok(Response {
                        headers,
                        response,
                        status_code,
                    }))
                }
                500..=599 => {
                    let bytes = resp.bytes().await.map_err(|error| {
                        report!(error).change_context(ApiClientError::ResponseDecodingFailed)
                    })?;

                    Ok(Err(Response {
                        headers,
                        response: bytes,
                        status_code,
                    }))
                }

                400..=499 => {
                    let bytes = resp.bytes().await.map_err(|error| {
                        report!(error).change_context(ApiClientError::ResponseDecodingFailed)
                    })?;

                    Ok(Err(Response {
                        headers,
                        response: bytes,
                        status_code,
                    }))
                }
                _ => {
                    // info_log(
                    //     "UNEXPECTED_RESPONSE",
                    //     &json!("Unexpected response from server."),
                    // );
                    Err(report!(ApiClientError::UnexpectedServerResponse))
                }
            }
        })
        .await?
}
pub async fn call_connector_api(
    request: Request,
) -> CustomResult<Result<Response, Response>, ApiClientError> {
    let proxy_url = Some("http://localhost:8081");
    let client = create_client(proxy_url)?;
    let url =
        reqwest::Url::parse(&request.url).change_context(ApiClientError::UrlEncodingFailed)?;

    let request: reqwest::RequestBuilder = {
        match request.method {
            Method::Get => client.get(url),
            Method::Post => {
                let client = client.post(url);
                match request.body {
                    Some(RequestContent::Json(payload)) => client.json(&payload),
                    Some(RequestContent::FormUrlEncoded(payload)) => client.form(&payload),
                    Some(RequestContent::Xml(payload)) => {
                        // For XML content, we need to extract the XML string properly
                        // The payload implements a custom Serialize that generates XML content
                        let body = serde_json::to_string(&payload)
                            .change_context(ApiClientError::UrlEncodingFailed)?;

                        // Properly deserialize the JSON string to extract clean XML
                        let xml_body = if body.starts_with('"') && body.ends_with('"') {
                            // This is a JSON-encoded string, deserialize it properly
                            serde_json::from_str::<String>(&body)
                                .change_context(ApiClientError::UrlEncodingFailed)?
                        } else {
                            // This is already the raw body content
                            body
                        };
                        client.body(xml_body).header("Content-Type", "text/xml")
                    }
                    Some(RequestContent::FormData(form)) => client.multipart(form),
                    Some(RequestContent::RawBytes(payload)) => client.body(payload),
                    _ => client,
                }
            }
        }
    };

    let send_request = async {
        request.send().await.map_err(|error| {
            let api_error = match error {
                error if error.is_timeout() => ApiClientError::RequestTimeoutReceived,
                _ => ApiClientError::RequestNotSent(error.to_string()),
            };

            // info_log(
            //     "REQUEST_FAILURE",
            //     &json!(format!("Unable to send request to connector.",)),
            // );
            report!(api_error)
        })
    };

    let response = send_request.await;
    handle_response(response).await
}
pub async fn execute_connector_processing_step<F, ResourceCommonData, Req, Resp>(
    // proxy: &Proxy,
    connector: BoxedConnectorIntegrationV2<'static, F, ResourceCommonData, Req, Resp>,
    router_data: RouterDataV2<F, ResourceCommonData, Req, Resp>,
    all_keys_required: Option<bool>,
    // event_params: EventProcessingParams<'_>,
    // token_data: Option<TokenData>,
    call_connector_action: common_enums::CallConnectorAction,
    // test_context: Option<TestContext>,
    // api_tag: Option<String>,
) -> CustomResult<(), ConnectorError>
where
    F: Clone + 'static,
    // T: FlowIntegrity,
    Req: Clone + 'static + std::fmt::Debug,
    Resp: Clone + 'static + std::fmt::Debug,
    ResourceCommonData: Clone + 'static, //     + RawConnectorRequestResponse
                                         //     + ConnectorResponseHeaders
                                         //     + ConnectorRequestReference
                                         //     + AdditionalHeaders,
{
    let start = tokio::time::Instant::now();
    let result = match call_connector_action {
        common_enums::CallConnectorAction::Trigger => {
            let mut connector_request = connector.build_request_v2(&router_data.clone())?;

            let _ = match connector_request.as_ref() {
                Some(request) => {
                    let raw = extract_raw_connector_request(&request);
                    println!("{:?} RAW REQUEST", raw);
                }
                None => (),
            };

            let _ = match connector_request {
                Some(request) => {
                    let response = call_connector_api(request)
                        .await
                        .change_context(ConnectorError::RequestEncodingFailed)
                        .inspect_err(|err| {
                            println!("{} NETWORK ERROR", err);
                        });
                }
                None => (),
            };
        }
    };
    Ok(result)
}
